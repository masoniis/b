struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> camera : CameraUniform;

@group(1) @binding(0) var myTexture: texture_2d_array<f32>;
@group(1) @binding(1) var mySampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) texture_index: u32,
    @location(4) model_row_0: vec4<f32>,
    @location(5) model_row_1: vec4<f32>,
    @location(6) model_row_2: vec4<f32>,
    @location(7) model_row_3: vec4<f32>,
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
    let model_matrix = mat4x4<f32>(model_in.model_row_0, model_in.model_row_1,
        model_in.model_row_2, model_in.model_row_3);
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model_in.position, 1.0);
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
