use crate::simulation_world::time::FrameClock;
use crate::{
    simulation_world::camera::camera::CameraResource,
    simulation_world::input::resources::WindowSizeResource,
    simulation_world::input::{
        resources::CursorMovement, types::simulation_action::SimulationAction, ActionStateResource,
    },
};
use bevy_ecs::prelude::*;
use glam::{Mat4, Vec3};
use tracing::instrument;

#[instrument(skip_all)]
pub fn camera_control_system(
    // Input
    movement: Res<CursorMovement>,
    action_state: Res<ActionStateResource>,
    time: Res<FrameClock>,
    window: Res<WindowSizeResource>,

    // Output
    mut camera: ResMut<CameraResource>,
) {
    // Update camera position
    let velocity = camera.movement_speed * time.delta.as_secs_f32();
    let front = camera.front;
    let mut multiplier = 1.0;
    if action_state.is_ongoing(SimulationAction::MoveFaster) {
        multiplier = 5.0;
    }
    if action_state.is_ongoing(SimulationAction::MoveForward) {
        camera.position += front * velocity * multiplier;
    }
    let front = camera.front;
    if action_state.is_ongoing(SimulationAction::MoveBackward) {
        camera.position -= front * velocity * multiplier;
    }
    let right = camera.right;
    if action_state.is_ongoing(SimulationAction::MoveLeft) {
        camera.position -= right * velocity * multiplier;
    }
    let right = camera.right;
    if action_state.is_ongoing(SimulationAction::MoveRight) {
        camera.position += right * velocity * multiplier;
    }

    // Update camera pitch
    let mut xoffset = movement.get_mouse_delta().x as f32;
    let mut yoffset = movement.get_mouse_delta().y as f32;
    let constrain_pitch = true; // Assuming this is always true for now, or could be a resource/config

    xoffset *= camera.mouse_sensitivity;
    yoffset *= camera.mouse_sensitivity;

    camera.yaw += xoffset;
    camera.pitch -= yoffset;

    if constrain_pitch {
        if camera.pitch > 89.0 {
            camera.pitch = 89.0;
        }
        if camera.pitch < -89.0 {
            camera.pitch = -89.0;
        }
    }

    // Call the internal update_camera_vectors logic
    let yaw_radians = camera.yaw.to_radians();
    let pitch_radians = camera.pitch.to_radians();

    let x = yaw_radians.cos() * pitch_radians.cos();
    let y = pitch_radians.sin();
    let z = yaw_radians.sin() * pitch_radians.cos();

    camera.front = Vec3::new(x, y, z).normalize();
    camera.right = camera.front.cross(camera.world_up).normalize();
    camera.up = camera.right.cross(camera.front).normalize();

    // Handle scrolling
    let yoffset_scroll = movement.get_scroll_delta().y;
    camera.zoom -= yoffset_scroll;
    if camera.zoom < 1.0 {
        camera.zoom = 1.0;
    }
    if camera.zoom > 45.0 {
        camera.zoom = 45.0;
    }
    camera.projection_dirty = true;

    camera.view_matrix =
        Mat4::look_at_rh(camera.position, camera.position + camera.front, camera.up);

    // (projection matrix is expensive, only update if dirty)
    if camera.projection_dirty {
        camera.projection_matrix = Mat4::perspective_infinite_reverse_rh(
            camera.zoom.to_radians(),
            window.aspect_ratio(),
            1.0,
        );
        camera.projection_dirty = false;
    }
}
