struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera : CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) model_row_0: vec4<f32>,
    @location(3) model_row_1: vec4<f32>,
    @location(4) model_row_2: vec4<f32>,
    @location(5) model_row_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model_in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let model_matrix = mat4x4<f32>(model_in.model_row_0, model_in.model_row_1,
        model_in.model_row_2, model_in.model_row_3);
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model_in.position, 1.0);
    out.color = model_in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
