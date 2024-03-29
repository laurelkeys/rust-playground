// @Todo: continue from https://sotrh.github.io/learn-wgpu/intermediate/tutorial12-camera/#the-camera

use std::path::Path;

use cgmath::{Deg, InnerSpace, Matrix3, Matrix4, Quaternion, Rotation3, Vector3, Zero};
use wgpu::{include_wgsl, util::DeviceExt};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod assets;
mod camera;
mod model;
mod texture;

use crate::camera::{Camera, CameraController};
use crate::model::{DrawLight, DrawModel, Vertex};

/// Maps z coordinate values from `-1.0..=1.0` to `0.0..=1.0`.
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, // 1st column
    0.0, 1.0, 0.0, 0.0, // 2nd column
    0.0, 0.0, 0.5, 0.0, // 3rd column
    0.0, 0.0, 0.5, 1.0, // 4th column
);

//
// CameraUniform
//

// @Volatile: keep shader.wgsl and light.wgsl synced with this.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// Camera position in "world space" coordinates.
    // @Note: store 4 floats because of uniforms' 16 byte spacing requirement.
    world_position: [f32; 4],
    /// Combined view ("world to view") and projection ("view to clip") matrix.
    // @Note: we can't use cgmath directly with bytemuck, so we convert Matrix4.
    clip_from_world: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(world_position: [f32; 4]) -> Self {
        use cgmath::SquareMatrix;

        Self { world_position, clip_from_world: Matrix4::identity().into() }
    }

    /// Updates the combined "view projection" matrix uniform, which
    /// is used to transform world coordinates into clip coordinates.
    pub fn update_clip_from_world(&mut self, camera: &Camera) {
        self.world_position = camera.eye.to_homogeneous().into();
        // @Note: Wgpu's coordinate system uses NDC with the x- and y-axis in the range
        // [-1.0, 1.0], but with the z-axis ranging from 0.0 to 1.0. However, cgmath
        // uses the same convention as OpenGL (with z in [-1.0, 1.0] as well).
        self.clip_from_world =
            (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

//
// LightUniform
//

// @Volatile: keep shader.wgsl and light.wgsl synced with this.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    world_position: [f32; 3],
    _pad0: u32, // @Note: uniforms require 16 byte spacing, so we add padding
    color: [f32; 3],
    _pad1: u32, // ditto (see https://www.w3.org/TR/WGSL/#alignment-and-size)
}

impl LightUniform {
    fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { world_position: position, _pad0: 0, color, _pad1: 0 }
    }
}

//
// Instance, InstanceRaw
//

struct Instance {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        let rotation = Matrix4::from(self.rotation);
        let world_from_local = (Matrix4::from_translation(self.position) * rotation).into();
        let world_normal_from_local_normal = Matrix3::from_cols(
            rotation.x.truncate(), // 1st column
            rotation.y.truncate(), // 2nd column
            rotation.z.truncate(), // 3rd column
        )
        .into();

        InstanceRaw { world_from_local, world_normal_from_local_normal }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    /// `Instace` transform represented as a 4x4 "model" matrix, which
    /// takes the model's local coordinate system to world coordinates.
    world_from_local: [[f32; 4]; 4],
    // @Note: we only need the rotation component of the transformation
    // matrix for normals (as it doesn't make sense to translate them),
    // hence why we use a 3x3 instead of a 4x4 representation for it.
    world_normal_from_local_normal: [[f32; 3]; 3],
}

impl Vertex for InstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
            // @Note: a mat4 takes up 4 vertex slots as it is technically equivalent
            // to four vec4's... we will need to reassemble it in the shader then.
            // @Note: we start at slot 5 since `ModelVertex`'s desc() uses 0 to 4.
            5 => Float32x4, // world_from_local: [[f32; 4]; 4],
            6 => Float32x4, // ^
            7 => Float32x4, // ^
            8 => Float32x4, // ^
            // @Note: just like mat4 above, we represent a mat3 using three vec3's.
            9 => Float32x3, // world_normal_from_local_normal: [[f32; 3]; 3],
            10 => Float32x3, // ^
            11 => Float32x3, // ^
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBS,
        }
    }
}

//
// State
//

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    physical_size: winit::dpi::PhysicalSize<u32>,

    clear_color: wgpu::Color,

    light_render_pipeline: wgpu::RenderPipeline,
    render_pipeline: wgpu::RenderPipeline,

    camera: Camera,
    camera_controller: CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    light_uniform: LightUniform,
    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,

    depth_texture: texture::Texture,

    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,

    cube_model: model::Model,
    debug_material: model::Material,
}

fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader_desc: wgpu::ShaderModuleDescriptor,
    color_format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_buffers_layouts: &[wgpu::VertexBufferLayout],
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(shader_desc);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("render_pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_buffers_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // @Note: changing any of the 3 fields below requires support for specific `Features`.
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
            format,
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
        multiview: None,
    })
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
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) =
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        surface.configure(&device, &config);

        //
        // Texture bind group layout.
        //

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    // Diffuse texture:
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
                        // @Note: should match the above Texture's `sample_type.filterable`.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Normal map texture:
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // @Note: ditto.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        //
        // Camera setup and bind group layout.
        //

        let camera = Camera {
            eye: (0.0, 5.0, -10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            y_fov: Deg(45.0),
            z_near: 0.1,
            z_far: 100.0,
        };

        let mut camera_uniform = CameraUniform::new([0.0; 4]);
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
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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

        //
        // Light setup and bind group layout.
        //

        let light_uniform = LightUniform::new([2.0, 2.0, 2.0], [1.0, 1.0, 1.0]);

        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("light_buffer"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("light_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("light_bind_group"),
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
        });

        //
        // Render pipeline(s) configuration.
        //

        let light_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("light_render_pipeline_layout"),
                // Bind groups that this pipeline uses:
                bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                push_constant_ranges: &[],
            });

        let light_render_pipeline = create_render_pipeline(
            &device,
            &light_render_pipeline_layout,
            include_wgsl!("light.wgsl"),
            config.format,
            Some(texture::Texture::DEPTH_FORMAT),
            &[model::ModelVertex::desc()],
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                // Bind groups that this pipeline uses:
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_bind_group_layout,
                    &light_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = create_render_pipeline(
            &device,
            &render_pipeline_layout,
            include_wgsl!("shader.wgsl"),
            config.format,
            Some(texture::Texture::DEPTH_FORMAT),
            &[model::ModelVertex::desc(), InstanceRaw::desc()],
        );

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, Some("depth_texture"));

        const SPACE_BETWEEN_INSTANCES: f32 = 3.0;
        const INSTANCES_PER_ROW_COUNT: u32 = 10;
        const INSTANCES_PER_ROW_HALF_COUNT: f32 = 0.5 * INSTANCES_PER_ROW_COUNT as f32;

        let instances = (0..INSTANCES_PER_ROW_COUNT)
            .flat_map(|z| {
                (0..INSTANCES_PER_ROW_COUNT).map(move |x| {
                    let position = {
                        let x = SPACE_BETWEEN_INSTANCES * (x as f32 - INSTANCES_PER_ROW_HALF_COUNT);
                        let z = SPACE_BETWEEN_INSTANCES * (z as f32 - INSTANCES_PER_ROW_HALF_COUNT);
                        Vector3 { x, y: 0.0, z }
                    };

                    let rotation = if position.is_zero() {
                        Quaternion::from_axis_angle(Vector3::unit_z(), Deg(0.0))
                    } else {
                        Quaternion::from_axis_angle(position.normalize(), Deg(45.0))
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

        let cube_dir = Path::new("models").join("cube");
        let cube_obj_path = cube_dir.join("cube.obj");
        let cobble_diffuse_png_path = cube_dir.join("cobble-diffuse.png");
        let cobble_normal_png_path = cube_dir.join("cobble-normal.png");

        let cube_model = assets::load_model(
            cube_obj_path.into_os_string().to_str().unwrap(),
            &device,
            &queue,
            &texture_bind_group_layout,
        )
        .unwrap();

        let debug_material = {
            let diffuse_texture = assets::load_texture(
                cobble_diffuse_png_path.into_os_string().to_str().unwrap(),
                texture::TextureIsSrgb::Encoded,
                &device,
                &queue,
            )
            .unwrap();

            let normal_texture = assets::load_texture(
                cobble_normal_png_path.into_os_string().to_str().unwrap(),
                texture::TextureIsSrgb::Linear,
                &device,
                &queue,
            )
            .unwrap();

            model::Material::new(
                &device,
                &texture_bind_group_layout,
                "debug-material",
                diffuse_texture,
                normal_texture,
            )
        };

        Self {
            surface,
            device,
            queue,
            config,
            physical_size,
            clear_color: wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 },
            light_render_pipeline,
            render_pipeline,
            camera,
            camera_controller: CameraController::new(0.2),
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            light_uniform,
            light_buffer,
            light_bind_group,
            depth_texture,
            instances,
            instance_buffer,
            cube_model,
            debug_material,
        }
    }

    fn resize(&mut self, new_physical_size: winit::dpi::PhysicalSize<u32>) {
        if new_physical_size.width == 0 || new_physical_size.height == 0 {
            return;
        }

        self.physical_size = new_physical_size;
        self.camera.aspect = new_physical_size.width as f32 / new_physical_size.height as f32;

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

        let old_position = Vector3::from(self.light_uniform.world_position);
        let new_position = Quaternion::from_axis_angle(Vector3::unit_y(), Deg(1.0)) * old_position;
        self.light_uniform.world_position = new_position.into();
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.light_uniform]));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("encoder") });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(self.clear_color), store: true },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: true }),
                stencil_ops: None,
            }),
        });

        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        render_pass.set_pipeline(&self.light_render_pipeline);
        render_pass.draw_light_model(
            &self.cube_model,
            &self.camera_bind_group,
            &self.light_bind_group,
        );

        render_pass.set_pipeline(&self.render_pipeline);
        const USE_DEBUG_MATERIAL: bool = false;
        match USE_DEBUG_MATERIAL {
            true => render_pass.draw_model_instanced_with_material(
                &self.cube_model,
                &self.camera_bind_group,
                &self.light_bind_group,
                0..self.instances.len() as u32,
                &self.debug_material,
            ),
            false => render_pass.draw_model_instanced(
                &self.cube_model,
                &self.camera_bind_group,
                &self.light_bind_group,
                0..self.instances.len() as u32,
            ),
        }

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

//
// main
//

fn main() {
    pollster::block_on(run());
}

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window =
        WindowBuilder::new().with_title(env!("CARGO_PKG_NAME")).build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it.
                window.request_redraw();
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(wgpu::SurfaceError::Timeout) => eprintln!("Surface timeout"),
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.physical_size)
                    }
                }
            }

            Event::WindowEvent { ref event, window_id }
                if window_id == window.id() && !state.input(event) =>
            {
                match event {
                    //
                    // Window close events
                    //
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    }
                    | WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

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

            _ => {}
        }
    });
}
