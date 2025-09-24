use crate::ecs::resources::{CameraResource, InputResource, TimeResource, WindowResource};
use crate::graphics::webgpu_renderer::WebGpuRenderer;
use bevy_ecs::prelude::*;
use glam::{Mat4, Vec3};
use winit::keyboard::KeyCode;

pub fn camera_control_system(
    input: Res<InputResource>,
    time: Res<TimeResource>,
    window: Res<WindowResource>,
    mut camera: ResMut<CameraResource>,
    mut renderer: ResMut<WebGpuRenderer>,
) {
    // Update camera position
    let velocity = camera.movement_speed * time.since_last_update.as_secs_f32();
    let front = camera.front;
    let mut multiplier = 1.0;
    if input.is_key_down(KeyCode::ShiftLeft) {
        multiplier = 2.5;
    }
    if input.is_key_down(KeyCode::KeyW) {
        camera.position += front * velocity * multiplier;
    }
    let front = camera.front;
    if input.is_key_down(KeyCode::KeyS) {
        camera.position -= front * velocity * multiplier;
    }
    let right = camera.right;
    if input.is_key_down(KeyCode::KeyA) {
        camera.position -= right * velocity * multiplier;
    }
    let right = camera.right;
    if input.is_key_down(KeyCode::KeyD) {
        camera.position += right * velocity * multiplier;
    }

    // Update camera pitch
    let mut xoffset = input.mouse_delta.x as f32;
    let mut yoffset = input.mouse_delta.y as f32;
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
    let yoffset_scroll = input.scroll_delta.y;
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
        camera.projection_matrix =
            Mat4::perspective_rh_gl(camera.zoom.to_radians(), window.aspect_ratio(), 0.1, 1000.0);
        camera.projection_dirty = false;
    }

    renderer.update_camera(camera.projection_matrix * camera.view_matrix);
}
