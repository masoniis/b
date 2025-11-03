// Per-view data (Camera)
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Per-object data (Model Transforms)
struct ObjectData {
    model_matrix: mat4x4<f32>,
};
@group(2) @binding(0)
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
};
