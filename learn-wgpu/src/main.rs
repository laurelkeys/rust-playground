// @Todo: continue from https://sotrh.github.io/learn-wgpu/intermediate/tutorial12-camera/#the-camera
// @Todo: update wgpu 0.10 -> 0.11.

use std::path::Path;

use cgmath::{InnerSpace, Matrix3, Matrix4, Point3, Quaternion, Rotation3, Vector3, Zero};
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod model;
mod texture;

use crate::model::{DrawLight, DrawModel, Vertex};

//
// Camera, CameraUniform
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
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, // 1st column
    0.0, 1.0, 0.0, 0.0, // 2nd column
    0.0, 0.0, 0.5, 0.0, // 3rd column
    0.0, 0.0, 0.5, 1.0, // 4th column
);

impl Camera {
    /// Returns a matrix that transforms world coordinates to clip coordinates, e.g.:
    /// ```
    /// let world_point = ...
    /// let clip_from_world = Camera::build_view_projection_matrix();
    /// let clip_point = clip_from_world * world_point; // projection
    /// ```
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view_from_world = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let clip_from_view = cgmath::perspective(self.y_fov, self.aspect, self.z_near, self.z_far);
        clip_from_view * view_from_world
    }
}

// @Volatile: keep shader.wgsl and light.wgsl synced with this.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    /// Camera position in "world space" coordinates.
    // @Note: store 4 floats because of uniforms' 16 byte spacing requirement.
    world_position: [f32; 4],
    /// Combined view ("world to view") and projection ("view to clip") matrix.
    // @Note: we can't use cgmath directly with bytemuck, so we convert Matrix4.
    clip_from_world: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;

        Self { world_position: [0.0; 4], clip_from_world: Matrix4::identity().into() }
    }

    /// Updates the combined "view projection" matrix uniform, which
    /// is used to transform world coordinates into clip coordinates.
    fn update_clip_from_world(&mut self, camera: &Camera) {
        self.world_position = camera.eye.to_homogeneous().into();
        // @Note: Wgpu's coordinate system uses NDC with the x- and y-axis in the range
        // [-1.0, 1.0], but with the z-axis ranging from 0.0 to 1.0. However, cgmath
        // uses the same convention as OpenGL (with z in [-1.0, 1.0] as well).
        self.clip_from_world =
            (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

//
// CameraController
//

// @Todo: replace this with https://github.com/h3r2tic/dolly
// (then, it'd also make sense to replace cgmath with glam).
struct CameraController {
    speed: f32,
    is_pressed: IsPressed,
}

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
// LightUniform
//

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    world_position: [f32; 3],
    // @Note: since uniforms require 16 byte spacing, we add a padding field.
    _padding: u32,
    color: [f32; 3],
}

impl LightUniform {
    fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { world_position: position, _padding: 0, color }
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

        // @Todo: "truncate" rotation from a Matrix4 to a Matrix3, avoiding recomputation.
        let world_normal_from_local_normal = Matrix3::from(self.rotation).into();

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

impl model::Vertex for InstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // world_from_local: [[f32; 4]; 4],
                // @Note: a mat4 takes up 4 vertex slots as it is technically equivalent
                // to four vec4's... we will need to reassemble it in the shader then.
                // @Note: we start at slot 5 since `ModelVertex`'s desc() uses 0 to 4.
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4 * 2]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4 * 3]>() as wgpu::BufferAddress,
                    shader_location: 8,
                },
                // world_normal_from_local_normal: [[f32; 3]; 3],
                // @Note: just like mat4 above, we represent a mat3 using three vec3's.
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 4 * 4]>() as wgpu::BufferAddress,
                    shader_location: 9,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 4 * 4 + 3]>() as wgpu::BufferAddress,
                    shader_location: 10,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 4 * 4 + 3 * 2]>() as wgpu::BufferAddress,
                    shader_location: 11,
                },
            ],
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
    let shader = device.create_shader_module(&shader_desc);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("render_pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "main",
            buffers: vertex_buffers_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: color_format,
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
                        ty: wgpu::BindingType::Sampler {
                            // @Volatile: must be true if the `sample_type` of the texture is:
                            // `TextureSampleType::Float { filterable: true }`, as it is above.
                            filtering: true,
                            // @Note: this is only for `TextureSampleType::Depth`.
                            comparison: false,
                        },
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
                        ty: wgpu::BindingType::Sampler { filtering: true, comparison: false },
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
            wgpu::ShaderModuleDescriptor {
                label: Some("light.wgsl"),
                source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
            },
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
            wgpu::ShaderModuleDescriptor {
                label: Some("shader.wgsl"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            },
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

        let assets_dir = Path::new(env!("OUT_DIR")).join("assets");
        let cube_model_dir = assets_dir.join("models").join("cube");

        let cube_model = model::Model::load(
            &device,
            &queue,
            &texture_bind_group_layout,
            cube_model_dir.join("cube.obj"),
        )
        .unwrap();

        let debug_material = {
            let diffuse_texture = texture::Texture::load(
                &device,
                &queue,
                cube_model_dir.join("cobble-diffuse.png"),
                texture::TextureIsSrgb::Encoded,
            )
            .unwrap();

            let normal_texture = texture::Texture::load(
                &device,
                &queue,
                cube_model_dir.join("cobble-normal.png"),
                texture::TextureIsSrgb::Linear,
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
        let new_position =
            Quaternion::from_axis_angle(Vector3::unit_y(), cgmath::Deg(1.0)) * old_position;
        self.light_uniform.world_position = new_position.into();
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.light_uniform]));
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
