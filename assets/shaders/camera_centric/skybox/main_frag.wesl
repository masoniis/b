import super::super::lib::environment_uniform::{env_params, calculate_sky_color};
import super::super::lib::camera_uniform::{camera};

@fragment
fn fs_main(
    @location(0) clip_position: vec4<f32>,
) -> @location(0) vec4<f32> {
    let world_pos_h = camera.inverse_view_proj * clip_position;

		let dir = normalize(world_pos_h.xyz);

    let sky_color = calculate_sky_color(dir, env_params);

    return vec4<f32>(sky_color, 1.0);
}
