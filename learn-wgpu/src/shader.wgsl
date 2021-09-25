//
// Vertex shader
//

// @Note: any structure used as a `uniform` must be annotated
// with `[[block]]`, as: `var<uniform> camera: CameraUniform`.

[[block]]
struct CameraUniform {
    clip_from_world: mat4x4<f32>; // combined "view projection" matrix
};

[[group(1), binding(0)]] // `camera_bind_group`
var<uniform> camera: CameraUniform;

struct InstanceInput {
    [[location(5)]] world_from_local_0: vec4<f32>;
    [[location(6)]] world_from_local_1: vec4<f32>;
    [[location(7)]] world_from_local_2: vec4<f32>;
    [[location(8)]] world_from_local_3: vec4<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] texcoord: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>; // @Note: analogous to gl_Position
    [[location(0)]] texcoord: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let world_from_local = mat4x4<f32>(
        instance.world_from_local_0,
        instance.world_from_local_1,
        instance.world_from_local_2,
        instance.world_from_local_3
    );

    var out: VertexOutput;
    out.clip_position = camera.clip_from_world * world_from_local * vec4<f32>(model.position, 1.0);
    out.texcoord = model.texcoord;
    return out;
}

//
// Fragment shader
//

[[group(0), binding(0)]] // `diffuse_bind_group`
var t_diffuse: texture_2d<f32>;

[[group(0), binding(1)]] // `diffuse_bind_group`
var s_diffuse: sampler;

[[stage(fragment)]]
fn main(
    in: VertexOutput
) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.texcoord);
}
