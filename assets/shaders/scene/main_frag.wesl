import package::scene::common::{myTexture, mySampler, VertexOutput};

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color: vec4<f32> = textureSample(myTexture, mySampler, in.tex_coords, in.texture_index);
    return vec4<f32>(in.color * texture_color.rgb, 1.0);
}
