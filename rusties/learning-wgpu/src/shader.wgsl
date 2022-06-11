struct CameraUniform {
    view_proj_matrix: mat4x4<f32>;
};

[[group(1), binding(0)]]
var<uniform> camera: CameraUniform;

struct CubeUniform {
    model_matrix: mat4x4<f32>;
};

[[group(2), binding(0)]]
var<uniform> cube: CubeUniform;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(vertex_input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.tex_coords = vertex_input.tex_coords;
    out.clip_position = camera.view_proj_matrix * cube.model_matrix * vec4<f32>(vertex_input.position, 1.0);

    return out;
}

[[group(0), binding(0)]]
var tree_texture: texture_2d<f32>;
[[group(0), binding(1)]]
var tree_texture_sampler: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var sampled = textureSample(tree_texture, tree_texture_sampler, in.tex_coords);

    return sampled;
}