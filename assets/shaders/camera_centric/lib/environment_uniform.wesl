struct EnvironmentUniform {
    sun_direction: vec3<f32>,
    horizon_color: vec3<f32>,
    zenith_color: vec3<f32>,
};

@group(1) @binding(0)
var<uniform> env_params: EnvironmentUniform;

fn calculate_sky_color(dir: vec3<f32>, env: EnvironmentUniform) -> vec3<f32> {
    let t = (dir.y + 1.0) * 0.5;
    let t_pow = pow(t, 0.75);
    let sky_color = mix(env.horizon_color, env.zenith_color, t_pow);

    return sky_color;
}
