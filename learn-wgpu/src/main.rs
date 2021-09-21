// @Todo: continue from https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/#accessing-files-in-the-res-folder

use cgmath::{InnerSpace, Matrix4, Point3, Quaternion, Rotation3, Vector3, Zero};
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
            step_mode: wgpu::VertexStepMode::Vertex,
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
// CameraUniform.
//

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    /// Combined view ("world to view") and projection ("view to clip") matrix.
    // @Note: we can't use cgmath directly with bytemuck, so we convert Matrix4.
    clip_from_world: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;

        Self { clip_from_world: Matrix4::identity().into() }
    }

    /// Updates the combined "view projection" matrix uniform, which
    /// is used to transform world coordinates into clip coordinates.
    fn update_clip_from_world(&mut self, camera: &Camera) {
        self.clip_from_world = camera.build_view_projection_matrix().into();
    }
}

//
// CameraController.
//

use bitflags::bitflags;

bitflags! {
    #[derive(Default)] // empty()
    struct IsPressed: u32 {
        const UP       = 0b000001;
        const DOWN     = 0b000010;
        const LEFT     = 0b000100;
        const RIGHT    = 0b001000;
        const FORWARD  = 0b010000;
        const BACKWARD = 0b100000;
    }
}

// @Todo: replace this with https://github.com/h3r2tic/dolly
// (then, it'd also make sense to replace cgmath with glam).
struct CameraController {
    speed: f32,
    is_pressed: IsPressed,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self { speed, is_pressed: IsPressed::default() }
    }

    fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput { state, virtual_keycode: Some(keycode), .. },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space => {
                        self.is_pressed.set(IsPressed::UP, is_pressed);
                    }
                    VirtualKeyCode::LShift => {
                        self.is_pressed.set(IsPressed::DOWN, is_pressed);
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_pressed.set(IsPressed::LEFT, is_pressed);
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_pressed.set(IsPressed::RIGHT, is_pressed);
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_pressed.set(IsPressed::FORWARD, is_pressed);
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_pressed.set(IsPressed::BACKWARD, is_pressed);
                    }
                    _ => return false,
                }
                true
            }
            _ => false,
        }
    }

    fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();
        let forward = forward / forward_mag; // forward.normalize()

        if self.is_pressed.contains(IsPressed::FORWARD) {
            if forward_mag > self.speed {
                camera.eye += forward * self.speed;
            } else {
                // Do nothing.
            }
        }
        if self.is_pressed.contains(IsPressed::BACKWARD) {
            camera.eye -= forward * self.speed;
        }

        let right = forward.cross(camera.up);
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_pressed.contains(IsPressed::LEFT) {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
        if self.is_pressed.contains(IsPressed::RIGHT) {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
    }
}

//
// InstanceRaw.
//

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    /// `Instace` transform represented as a 4x4 "model" matrix, which
    /// takes the model's local coordinate system to world coordinates.
    world_from_local: [[f32; 4]; 4],
}

impl InstanceRaw {
    /// Returns a descriptor of how the vertex buffer is interpreted.
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // world_from_local: [[f32; 4]; 4],
                // @Note: a mat4 takes up 4 vertex slots as it is technically equivalent
                // to four vec4's... we will need to reassemble it in the shader then.
                // @Note: we are starting at a higher slot than we currently need to,
                // so that we leave space for using new locations in `Vertex` later.
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 5 + 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4 * 2]>() as wgpu::BufferAddress,
                    shader_location: 5 + 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4 * 3]>() as wgpu::BufferAddress,
                    shader_location: 5 + 3,
                },
            ],
        }
    }
}

//
// Instance.
//

struct Instance {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        let world_from_local =
            Matrix4::from_translation(self.position) * Matrix4::from(self.rotation);
        InstanceRaw { world_from_local: world_from_local.into() }
    }
}

const INSTANCES_PER_ROW_COUNT: u32 = 10;
const INSTANCES_TOTAL_COUNT: u32 = INSTANCES_PER_ROW_COUNT * INSTANCES_PER_ROW_COUNT;
const INSTANCE_DISPLACEMENT: Vector3<f32> =
    Vector3::new(0.5 * INSTANCES_PER_ROW_COUNT as f32, 0.0, 0.5 * INSTANCES_PER_ROW_COUNT as f32);

//
// State.
//

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    // @Note: size represented in physical pixels (as opposed to logical pixels).
    physical_size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices_count: u32,
    index_buffer: wgpu::Buffer,
    indices_count: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    depth_texture: texture::Texture,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: CameraController,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
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
        //  - all() = PRIMARY + SECONDARY
        //  - PRIMARY = Vulkan + Metal + DX12 + Browser WebGPU
        //  - SECONDARY = OpenGL + DX11
        let instance = wgpu::Instance::new(wgpu::Backends::all());

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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
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

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, Some("depth_texture"));

        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(), // 1 unit up and 2 units back the screen
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            y_fov: cgmath::Deg(45.0),
            z_near: 0.1,
            z_far: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_clip_from_world(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("camera_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(WGSL_SHADER_SOURCE_CODE.into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: VERT_SHADER_ENTRY_POINT,
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: FRAG_SHADER_ENTRY_POINT,
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0, // enables all samples
                alpha_to_coverage_enabled: false,
            },
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let instances = (0..INSTANCES_PER_ROW_COUNT)
            .flat_map(|z| {
                (0..INSTANCES_PER_ROW_COUNT).map(move |x| {
                    let position = Vector3 { x: x as f32, y: 0.0, z: z as f32 };
                    let rotation = if position.is_zero() {
                        Quaternion::from_axis_angle(Vector3::unit_z(), cgmath::Deg(0.0))
                    } else {
                        Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                    };
                    Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("instance_buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            physical_size,
            clear_color: wgpu::Color::WHITE,
            render_pipeline,
            vertex_buffer,
            vertices_count: VERTICES.len() as u32,
            index_buffer,
            indices_count: INDICES.len() as u32,
            diffuse_bind_group,
            diffuse_texture,
            depth_texture,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller: CameraController::new(0.2),
            instances,
            instance_buffer,
        }
    }

    fn resize(&mut self, new_physical_size: winit::dpi::PhysicalSize<u32>) {
        if new_physical_size.width == 0 || new_physical_size.height == 0 {
            return;
        }

        self.physical_size = new_physical_size;

        // Reconfigure the surface for presentation.
        self.config.width = new_physical_size.width;
        self.config.height = new_physical_size.height;
        self.surface.configure(&self.device, &self.config);

        // Recreate the depth texture, since we have changed config,
        // and depth_texture must be the same size as the surface's.
        self.depth_texture = texture::Texture::create_depth_texture(
            &self.device,
            &self.config,
            Some("depth_texture"),
        );
    }

    /// Indicates whether or not an event has been fully processed.
    fn input(&mut self, event: &WindowEvent) -> bool {
        // @Refactor: it is not clear that this returns a bool
        // (or, for that matter, what that value even means).
        self.camera_controller.process_events(event)
    }

    fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_clip_from_world(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // @Fixme: doing this in one go doesn't work (why?):
        //  |
        //  |   let view = self
        //  |       .surface
        //  |       .get_current_frame()?
        //  |       .output
        //  |       .texture
        //  |       .create_view(&wgpu::TextureViewDescriptor::default());
        //
        let output = self.surface.get_current_frame()?.output;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("encoder") });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(self.clear_color), store: true },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: true }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // group 0
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]); // group 1
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // slot 0
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..)); // slot 1
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices_count, 0, 0..self.instances.len() as u32);

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
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.physical_size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
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
