// Your existing camera uniform struct
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

// A new uniform struct just for the model matrix
struct ModelUniform {
    model_matrix: mat4x4<f32>,
};

// Your existing bind groups
@group(0) @binding(0) var<uniform> camera : CameraUniform;
@group(1) @binding(0) var myTexture: texture_2d_array<f32>;
@group(1) @binding(1) var mySampler: sampler;
@group(2) @binding(0) var<uniform> model: ModelUniform;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) texture_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) texture_index: u32,
};

@vertex
fn vs_main(model_in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * model.model_matrix * vec4<f32>(model_in.position, 1.0);
    out.color = model_in.color;
    out.tex_coords = model_in.tex_coords;
    out.texture_index = model_in.texture_index;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color: vec4<f32> = textureSample(myTexture, mySampler, in.tex_coords, in.texture_index);
    return vec4<f32>(in.color * texture_color.rgb, 1.0);
}
