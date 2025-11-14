use crate::prelude::*;
use crate::{
    render_world::{
        global_extract::RenderCameraResource,
        graphics_context::resources::RenderQueue,
        passes::world::shadow_pass::extract::ExtractedSun,
        passes::world::shadow_pass::startup::{ShadowViewBuffer, ShadowViewData},
    },
    simulation_world::player::CAMERA_NEAR_PLANE,
};
use bevy_ecs::prelude::*;
use glam::Vec4Swizzles;

/// The max distance at which shadows render
const SHADOW_DISTANCE: f32 = 256.0;

/// Calculates the sun's view/projection matrix and uploads it to the GPU buffer.
#[instrument(skip_all)]
pub fn update_shadow_view_buffer_system(
    // Input
    view_buffer: Res<ShadowViewBuffer>,
    camera: Res<RenderCameraResource>,
    sun: Res<ExtractedSun>,

    // Output (writing buffer to queue)
    queue: Res<RenderQueue>,
) {
    // create sun view matrix that points towards the camera
    let sun_direction = Vec3::from_array(sun.direction).normalize_or_zero();
    let light_target = camera.world_position;
    let light_position = light_target + sun_direction * 2048.0; // place light "far away" from camera
    let light_view_matrix = Mat4::look_at_rh(light_position, light_target, Vec3::Y);

    // camera inverse view projection
    let view_proj = camera.projection_matrix * camera.view_matrix;
    let inv_view_proj = view_proj.inverse();

    // INFO: ------------------------------
    //         frustum bounding box
    // ------------------------------------

    // far plane is close to 0 (far away for inverse z proj)
    let z_ndc_far = CAMERA_NEAR_PLANE / SHADOW_DISTANCE;
    let frustum_corners_ndc = [
        // near plane (z=1.0)
        vec4(-1.0, -1.0, 1.0, 1.0),
        vec4(1.0, -1.0, 1.0, 1.0),
        vec4(-1.0, 1.0, 1.0, 1.0),
        vec4(1.0, 1.0, 1.0, 1.0),
        // far plane (z=z_ndc_far)
        vec4(-1.0, -1.0, z_ndc_far, 1.0),
        vec4(1.0, -1.0, z_ndc_far, 1.0),
        vec4(-1.0, 1.0, z_ndc_far, 1.0),
        vec4(1.0, 1.0, z_ndc_far, 1.0),
    ];

    // find the bounding box in sun-camera space
    let mut min_light_space = Vec3::splat(f32::MAX);
    let mut max_light_space = Vec3::splat(f32::MIN);

    for &corner_ndc in frustum_corners_ndc.iter() {
        // ndc -> world space
        let world_pos_w = inv_view_proj * corner_ndc;
        let world_pos = world_pos_w.xyz() / world_pos_w.w;

        // world space -> sun camera space
        let light_space_pos_w = light_view_matrix * world_pos.extend(1.0);
        let light_space_pos = light_space_pos_w.xyz();

        // find the min/max of the box in sun space
        min_light_space = min_light_space.min(light_space_pos);
        max_light_space = max_light_space.max(light_space_pos);
    }

    // INFO: ---------------------------------
    //         shadow ortho projection
    // ---------------------------------------

    // looking down the sun'z -Z axis (as if we are sun)
    // max.z is the nearest point to the sun
    // min.z is the furthest

    let near_plane = -max_light_space.z - 100.0; // 100.0 buffer to avoid clipping
    let far_plane = -min_light_space.z + 100.0;

    let light_proj_matrix = Mat4::orthographic_rh(
        min_light_space.x,
        max_light_space.x, // left, right
        min_light_space.y,
        max_light_space.y, // bottom, top
        near_plane,
        far_plane,
    );

    // INFO: ---------------------
    //         upload data
    // ---------------------------

    let light_view_proj_matrix = light_proj_matrix * light_view_matrix;
    let shadow_data = ShadowViewData {
        light_view_proj_matrix: light_view_proj_matrix.to_cols_array(),
    };

    queue.write_buffer(&view_buffer.buffer, 0, bytemuck::cast_slice(&[shadow_data]));
}
