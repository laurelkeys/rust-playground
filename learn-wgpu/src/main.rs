// @Todo: continue from https://sotrh.github.io/learn-wgpu/beginner/tutorial4-buffer/#the-index-buffer

use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
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
                // color: [f32; 3],
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

const WGSL_SHADER_SOURCE_CODE: &str = include_str!("shader.wgsl");
const VERT_SHADER_ENTRY_POINT: &str = "main"; // [[stage(vertex)]]
const FRAG_SHADER_ENTRY_POINT: &str = "main"; // [[stage(fragment)]]

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    // @Note: size represented in physical pixels (as opposed to logical pixels).
    physical_size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices_count: u32,
}

impl State {
    // @Note: these two ways of writing an async new() function would be equivalent:
    //  |
    //  |   async fn new(window: &Window) -> Self { ... }
    //  |
    //  |   fn new(window: &'_ Window) -> impl Future<Output = Self> + '_ { async { ... } }
    //
    // So, if to be "more explicit" about it, we could have written the later version.

    async fn new(window: &'_ Window) -> Self {
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

        let clear_color = wgpu::Color::WHITE;

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(WGSL_SHADER_SOURCE_CODE.into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
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
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        Self {
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
            physical_size,
            clear_color,
            render_pipeline,
            vertex_buffer,
            vertices_count: VERTICES.len() as u32,
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
            label: Some("Render Encoder"), // @Note: debug label (used for graphics debuggers)
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(self.clear_color), store: true },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // slot 0
        render_pass.draw(0..self.vertices_count, 0..1); // 3 vertices, 1 instance

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
