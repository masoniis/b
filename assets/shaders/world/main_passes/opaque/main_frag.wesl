import package::world::main_passes::lib::geometry_bindings::{
  texture_array, texture_sampler, camera, environment, shadow_camera, shadow_map, shadow_sampler
};
import package::world::shadow::lib::calculate_shadow::{calculate_shadow_from_world};

@fragment
fn fs_main(
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) @interpolate(flat) texture_index: u32,
    @location(4) world_pos: vec3<f32>,
) -> @location(0) vec4<f32> {
    // INFO: --------------------------
    //         texture sampling
    // --------------------------------

    let texture_color: vec4<f32> = textureSample(
        texture_array, texture_sampler, tex_coords, texture_index
    );
    let base_color = texture_color.rgb * color;

    // INFO: ------------------------------
    //         directional lighting
    // ------------------------------------

    let L = normalize(environment.sun_direction);
    let N = normalize(normal);
    let ambient_strength = 0.2;

    var lit_color: vec3<f32>;

    let shadow_factor: f32 = calculate_shadow_from_world(
        world_pos, shadow_camera, shadow_map, shadow_sampler
    );
    if shadow_factor == 1.0 {
        // not in shadow, apply full lighting
        let diffuse_factor = max(dot(N, L), 0.0);
        let lighting_factor = ambient_strength + (1.0 - ambient_strength) * diffuse_factor;
        lit_color = base_color * lighting_factor;
    } else {
        // in shadow, apply only ambient light
        lit_color = base_color * ambient_strength;
    }

    // INFO: -------------
    //         fog
    // -------------------

    let distance_from_camera = distance(world_pos, camera.world_position);
    let fog_amount: f32 = smoothstep(
        camera.render_distance * 0.75, camera.render_distance, distance_from_camera
    );
    let final_color_rgb = mix(lit_color, environment.horizon_color, fog_amount);

    return vec4<f32>(final_color_rgb, texture_color.a);
}
