// @Todo: continue from https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/

use cgmath::{EuclideanSpace, Matrix4, Point3, Vector3, Zero};
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod texture;

//
// Vertex.
//

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    texcoord: [f32; 2],
}

impl Vertex {
    /// Returns a descriptor of how the vertex buffer is interpreted.
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                // position: [f32; 3],
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                // texcoord: [f32; 2],
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    // @Note: flip `texcoord`'s y-axis since Wgpu's world coordinates have it pointing up (not down).
    Vertex { position: [-0.0868241 ,  0.49240386, 0.0], texcoord: [0.4131759   , 1.0 - 0.99240386 ], }, // A
    Vertex { position: [-0.49513406,  0.06958647, 0.0], texcoord: [0.0048659444, 1.0 - 0.56958646 ], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], texcoord: [0.28081453  , 1.0 - 0.050602943], }, // C
    Vertex { position: [ 0.35966998, -0.3473291 , 0.0], texcoord: [0.85967     , 1.0 - 0.15267089 ], }, // D
    Vertex { position: [ 0.44147372,  0.2347359 , 0.0], texcoord: [0.9414737   , 1.0 - 0.7347359  ], }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];

const WGSL_SHADER_SOURCE_CODE: &str = include_str!("shader.wgsl");
const VERT_SHADER_ENTRY_POINT: &str = "main"; // [[stage(vertex)]]
const FRAG_SHADER_ENTRY_POINT: &str = "main"; // [[stage(fragment)]]

//
// Camera.
//

struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    /// Camera aspect ratio.
    aspect: f32,
    /// Vertical field of view.
    y_fov: cgmath::Deg<f32>,
    /// Near clipping distance.
    z_near: f32,
    /// Far clipping distance.
    z_far: f32,
}

/// Maps z coordinate values from `-1.0..=1.0` to `0.0..=1.0`.
pub const WGPU_CLIP_FROM_OPENGL_CLIP: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, // 1st column
    0.0, 1.0, 0.0, 0.0, // 2nd column
    0.0, 0.0, 0.5, 0.0, // 3rd column
    0.0, 0.0, 0.5, 1.0, // 4th column
);

impl Camera {
    /// Returns a matrix that transforms world coordinates to clip coordinates, e.g.:
    /// ```rust
    /// let world_point = ...
    /// let clip_from_world = Camera::build_view_projection_matrix();
    /// let clip_point = clip_from_world * world_point; // projection
    /// ```
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view_from_world = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let clip_from_view = cgmath::perspective(self.y_fov, self.aspect, self.z_near, self.z_far);
        // @Note: Wgpu's coordinate system uses NDC with the x- and y-axis in the range
        // [-1.0, 1.0], but with the z-axis ranging from 0.0 to 1.0. However, cgmath
        // uses the same convention as OpenGL (with z in [-1.0, 1.0] as well).
        WGPU_CLIP_FROM_OPENGL_CLIP * clip_from_view * view_from_world
    }
}

//
// State.
//

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // @Note: size represented in physical pixels (as opposed to logical pixels).
    physical_size: winit::dpi::PhysicalSize<u32>,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices_count: u32,
    index_buffer: wgpu::Buffer,
    indices_count: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    camera: Camera,
}

impl State {
    // @Note: these two ways of writing an async new() function would be equivalent:
    //  |
    //  |   async fn new(window: &Window) -> Self { ... }
    //  |
    //  |   fn new(window: &'_ Window) -> impl Future<Output = Self> + '_ { async { ... } }
    //
    // So, to be "more explicit" about it, we could have written the second version.

    async fn new(window: &Window) -> Self {
        let physical_size = window.inner_size();

        // @Note: the instance is a handle to our GPU.
        // PRIMARY   = Vulkan + Metal + DX12 + Browser WebGPU
        // SECONDARY = OpenGL + DX11
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // @Safety: `window` is a valid object to create a surface upon.
        let surface = unsafe { instance.create_surface(window) };

        // @Robustness: these options aren't guaranteed to work for all
        // devices (but they should work for most of them).
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) =
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            // @Volatile: must be true if the `sample_type` of the texture is:
                            // `TextureSampleType::Float { filterable: true }`, as it is above.
                            filtering: true,
                            // @Note: this is only for `TextureSampleType::Depth`.
                            comparison: false,
                        },
                        count: None,
                    },
                ],
            });

        let diffuse_bytes = include_bytes!("../assets/images/happy-tree.png");
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, Some("diffuse_texture"))
                .unwrap();
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("diffuse_bind_group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
        });

        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(), // 1 unit up and 2 units back the screen
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: swap_chain_desc.width as f32 / swap_chain_desc.height as f32,
            y_fov: cgmath::Deg(45.0),
            z_near: 0.1,
            z_far: 100.0,
        };

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(WGSL_SHADER_SOURCE_CODE.into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: VERT_SHADER_ENTRY_POINT,
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: FRAG_SHADER_ENTRY_POINT,
                targets: &[wgpu::ColorTargetState {
                    format: swap_chain_desc.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Requires Features::DEPTH_CLAMPING to be set to true.
                clamp_depth: false,
                // Requires Features::NON_FILL_POLYGON_MODE if not set to Fill.
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::CONSERVATIVE_RASTERIZATION to be set to true.
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0, // enables all samples
                alpha_to_coverage_enabled: false,
            },
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });

        Self {
            surface,
            device,
            queue,
            physical_size,
            swap_chain_desc,
            swap_chain,
            clear_color: wgpu::Color::WHITE,
            render_pipeline,
            vertex_buffer,
            vertices_count: VERTICES.len() as u32,
            index_buffer,
            indices_count: INDICES.len() as u32,
            diffuse_bind_group,
            diffuse_texture,
            camera,
        }
    }

    fn resize(&mut self, new_physical_size: winit::dpi::PhysicalSize<u32>) {
        if new_physical_size.width == 0 || new_physical_size.height == 0 {
            return;
        }

        self.physical_size = new_physical_size;

        // Recreate the swap chain.
        self.swap_chain_desc.width = new_physical_size.width;
        self.swap_chain_desc.height = new_physical_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.swap_chain_desc);
    }

    /// Indicates whether or not an event has been fully processed.
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.clear_color = wgpu::Color {
                    r: position.x / self.physical_size.width as f64,
                    g: position.y / self.physical_size.height as f64,
                    ..wgpu::Color::WHITE
                };
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        // Do nothing.
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"), // @Note: debug label (used for graphics debuggers)
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(self.clear_color), store: true },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // group 0
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // slot 0
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices_count, 0, 0..1);

        // @Note: begin_render_pass() borrows `encoder` as `&mut self`, so we need
        // to release this mutable borrow before being able to call finish() on it.
        drop(render_pass);

        // @Note: submit() will accept anything that implements `IntoIter`, e.g.:
        //  |
        //  |   self.queue.submit(std::iter::once(encoder.finish()));
        //
        self.queue.submit([encoder.finish()]);

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // @Note: since main() can't be async, we need to block:
    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.physical_size),
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // Other errors (Timeout and Outdated) should be resolved by the next frame.
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it.
                window.request_redraw();
            }
            Event::WindowEvent { ref event, window_id }
                if window_id == window.id() && !state.input(event) =>
            {
                match event {
                    //
                    // Window close events
                    //
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    //
                    // Window resize events
                    //
                    WindowEvent::Resized(new_physical_size) => {
                        state.resize(*new_physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }

                    _ => {}
                }
            }
            _ => (),
        }
    });
}
