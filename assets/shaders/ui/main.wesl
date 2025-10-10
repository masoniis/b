// Per-view data
@group(0) @binding(0)
var<uniform> camera: mat4x4<f32>;

// Per-material data
struct Material {
    color: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> material: Material;

// Per-object data
struct Object {
    model: mat4x4<f32>,
};
@group(2) @binding(0)
var<storage, read> objects: array<Object>;

// Vertex shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let model = objects[instance_index].model;
    out.clip_position = camera * model * vec4<f32>(position, 0.0, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return material.color;
}
