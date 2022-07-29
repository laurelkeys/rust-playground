// @Volatile: sync with `CameraUniform` from main.rs.
struct CameraUniform {
    world_position: vec4<f32>,
    clip_from_world: mat4x4<f32>,
}

// @Volatile: sync with `LightUniform` from main.rs.
struct LightUniform {
    world_position: vec3<f32>,
    // _pad0: u32,
    color: vec3<f32>,
    // _pad1: u32,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

//
// Resource bindings
//

@group(0) @binding(0) var<uniform> camera: CameraUniform;

@group(1) @binding(0) var<uniform> light: LightUniform;

//
// Vertex shader
//

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let scale = 0.25;
    let world_position = model.position * scale + light.world_position;

    var out: VertexOutput;
    out.clip_position = camera.clip_from_world * vec4<f32>(world_position, 1.0);
    out.color = light.color;
    return out;
}

//
// Fragment shader
//

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
