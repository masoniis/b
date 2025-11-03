import package::opaque::common::{myTexture, mySampler};
import package::lib::sky_uniform::{SkyUniform};

@group(3) @binding(0)
var<uniform> sky_params: SkyUniform;

@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) @interpolate(flat) texture_index: u32,
    @location(4) distance_from_camera: f32,
) -> @location(0) vec4<f32> {
    let texture_color: vec4<f32> = textureSample(myTexture, mySampler, tex_coords, texture_index);

		// TODO: smoothstep params should be placed sky params so that it can be adjusted by things like render distance
    let fog_amount = smoothstep(128.0, 256.0, distance_from_camera);
    let final_color = mix(texture_color, vec4(sky_params.horizon_color, 1.0), fog_amount);

    return final_color;
}
