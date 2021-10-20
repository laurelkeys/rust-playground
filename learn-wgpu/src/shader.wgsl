//
// Vertex shader
//

// @Note: any structure used as a `uniform` must be annotated
// with `[[block]]`, as: `var<uniform> camera: CameraUniform`.

// @Volatile: sync this with `CameraUniform` from main.rs.
[[block]]
struct CameraUniform {
    world_position: vec4<f32>;
    clip_from_world: mat4x4<f32>;
};

[[group(1), binding(0)]]
var<uniform> camera: CameraUniform;

[[block]]
struct LightUniform {
    world_position: vec3<f32>;
    color: vec3<f32>;
};

[[group(2), binding(0)]]
var<uniform> light: LightUniform;

struct InstanceInput {
    [[location(5)]] world_from_local_0: vec4<f32>;
    [[location(6)]] world_from_local_1: vec4<f32>;
    [[location(7)]] world_from_local_2: vec4<f32>;
    [[location(8)]] world_from_local_3: vec4<f32>;
    [[location(9)]] world_normal_from_local_normal_0: vec3<f32>;
    [[location(10)]] world_normal_from_local_normal_1: vec3<f32>;
    [[location(11)]] world_normal_from_local_normal_2: vec3<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] texcoord: vec2<f32>;
    [[location(2)]] normal: vec3<f32>;
    [[location(3)]] tangent: vec3<f32>;
    [[location(4)]] bitangent: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>; // @Note: analogous to gl_Position
    [[location(0)]] texcoord: vec2<f32>;
    [[location(1)]] tangent_position: vec3<f32>;
    [[location(2)]] tangent_view_position: vec3<f32>;
    [[location(3)]] tangent_light_position: vec3<f32>;
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

    let world_normal_from_local_normal = mat3x3<f32>(
        instance.world_normal_from_local_normal_0,
        instance.world_normal_from_local_normal_1,
        instance.world_normal_from_local_normal_2
    );

    // @Robustness: at the moment, everything is being computed in world space
    // instead of view space (which minimizes floating-point precision errors).
    let world_position = world_from_local * vec4<f32>(model.position, 1.0);
    let world_normal = world_normal_from_local_normal * model.normal;
    let world_tangent = world_normal_from_local_normal * model.tangent;
    let world_bitangent = world_normal_from_local_normal * model.bitangent;

    // Assemble the "TBN matrix", used to transforms vectors to tangent space.
    let tangent_from_world = transpose(mat3x3<f32>(
        normalize(world_tangent), // world_tangent,
        normalize(world_bitangent), // world_bitangent,
        normalize(world_normal), // world_normal,
    ));

    var out: VertexOutput;
    out.clip_position = camera.clip_from_world * world_position;
    out.texcoord = model.texcoord;
    out.tangent_position = tangent_from_world * world_position.xyz;
    out.tangent_view_position = tangent_from_world * camera.world_position.xyz;
    out.tangent_light_position = tangent_from_world * light.world_position;
    return out;
}

//
// Fragment shader
//

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[group(0), binding(2)]]
var t_normal: texture_2d<f32>;
[[group(0), binding(3)]]
var s_normal: sampler;

[[stage(fragment)]]
fn main(
    in: VertexOutput
) -> [[location(0)]] vec4<f32> {
    let view_dir = normalize(in.tangent_view_position - in.tangent_position); // normalize(camera.world_position.xyz - in.world_position);
    let light_dir = normalize(in.tangent_light_position - in.tangent_position); // normalize(light.world_position - in.world_position);
    let hafway_dir = normalize(view_dir + light_dir); // Blinn-Phong
    // let reflect_dir = reflect(-light_dir, in.world_normal); // Phong

    let object_normal = textureSample(t_normal, s_normal, in.texcoord);
    let tangent_normal = object_normal.xyz * 2.0 - 1.0;

    let object_color = textureSample(t_diffuse, s_diffuse, in.texcoord);
    let ambient_color = light.color * 0.1;
    let diffuse_color = light.color * max(dot(tangent_normal, light_dir), 0.0); // light.color * max(dot(in.world_normal, light_dir), 0.0);
    let specular_color = light.color * pow(max(dot(tangent_normal, hafway_dir), 0.0), 32.0); // light.color * pow(max(dot(in.world_normal, hafway_dir), 0.0), 32.0);
    // let specular_color = light.color * pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);

    let final_color = (ambient_color + diffuse_color + specular_color) * object_color.xyz;

    return vec4<f32>(final_color, object_color.a);
}
