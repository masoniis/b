import package::lib::sky_uniform::{SkyUniform};

@group(1) @binding(0)
var<uniform> sky_params: SkyUniform;

@fragment
fn fs_main(
    @location(0) clip_position: vec4<f32>,
) -> @location(0) vec4<f32> {
    return vec4<f32>(sky_params.horizon_color, 1.0);
}
