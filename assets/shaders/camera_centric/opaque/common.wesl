// Per-material data (Texture Atlas)
@group(2) @binding(0)
var myTexture: texture_2d_array<f32>;
@group(2) @binding(1)
var mySampler: sampler;

// Per-object data (Model Transforms)
struct ObjectData {
    model_matrix: mat4x4<f32>,
};
@group(3) @binding(0)
var<storage, read> objects: array<ObjectData>;

// Vertex input and output
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
    @location(4) texture_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) @interpolate(flat) texture_index: u32,
    @location(4) distance_from_camera: f32,
};
