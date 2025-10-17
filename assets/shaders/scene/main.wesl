import package::scene::common::{CameraUniform, ModelUniform, VertexInput, VertexOutput, myTexture, mySampler, camera, model};

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

