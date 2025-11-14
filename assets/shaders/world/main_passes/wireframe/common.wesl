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

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};
