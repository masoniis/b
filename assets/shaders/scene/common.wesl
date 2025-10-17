struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct ModelUniform {
    model_matrix: mat4x4<f32>,
};

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
