import super::super::lib::environment_uniform::{env_params};

@fragment
fn fs_main(
    @location(0) clip_position: vec4<f32>,
) -> @location(0) vec4<f32> {
    return vec4<f32>(env_params.horizon_color, 1.0);
}
