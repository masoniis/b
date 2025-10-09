// The bind group now contains a shared projection matrix and per-object data.
@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

struct InstanceData {
    model: mat4x4<f32>,
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> instance: InstanceData;

// Vertex shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    // Project the model into clip space.
    out.clip_position = projection * instance.model * vec4<f32>(position, 0.0, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // Just output the color from the uniform buffer
    return instance.color;
}
