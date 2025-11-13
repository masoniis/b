import super::super::lib::environment_uniform::{env_params, calculate_sky_color};
import super::super::lib::camera_uniform::{camera};

@fragment
fn fs_main(
    @location(0) clip_position: vec4<f32>,
) -> @location(0) vec4<f32> {
    let world_pos_h = camera.inverse_view_proj * clip_position;
    let camera_dir = normalize(world_pos_h.xyz);

    let sky_color = calculate_sky_color(camera_dir, env_params);

		// sun math
    let sun_color = vec3(1.0, 0.98, 0.9);
    let halo_color = vec3(1.0, 0.9, 0.7);

    let sun_dir_norm = normalize(env_params.sun_direction);
    let sun_dot = dot(camera_dir, sun_dir_norm);

    let sun_disk = pow(max(0.0, sun_dot), 1024.0) * sun_color;
    let sun_halo = pow(max(0.0, sun_dot), 64.0) * halo_color * 0.2; // 0.2 intensity

		// blend all
    let final_color = sky_color + sun_disk + sun_halo;
    return vec4<f32>(final_color, 1.0);
}
