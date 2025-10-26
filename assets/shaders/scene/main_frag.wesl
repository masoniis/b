import package::scene::common::{myTexture, mySampler};

@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) @interpolate(flat) texture_index: u32,
) -> @location(0) vec4<f32> {
		// currently frag color is entirely determined by the texture
    let texture_color: vec4<f32> = textureSample(myTexture, mySampler, tex_coords, texture_index);
    let diffuse_color = texture_color.rgb;
    return vec4<f32>(diffuse_color, 1.0);
}
