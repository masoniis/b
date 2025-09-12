use crate::ecs::resources::{input::InputResource, Camera, TimeResource, WindowResource};
use glam::{Mat4, Vec3};
use shred::{Read, System, SystemData, Write};
use winit::keyboard::KeyCode;

#[derive(SystemData)]
pub struct CameraControlSystemData<'a> {
    input: Read<'a, InputResource>,
    time: Read<'a, TimeResource>,
    window: Read<'a, WindowResource>,
    camera: Write<'a, Camera>,
}

pub struct CameraControlSystem;

impl<'a> System<'a> for CameraControlSystem {
    type SystemData = CameraControlSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // Update camera position
        let velocity = data.camera.movement_speed * data.time.delta_seconds;
        let front = data.camera.front;
        if data.input.pressed_keys.contains(&KeyCode::KeyW) {
            data.camera.position += front * velocity;
        }
        let front = data.camera.front;
        if data.input.pressed_keys.contains(&KeyCode::KeyS) {
            data.camera.position -= front * velocity;
        }
        let right = data.camera.right;
        if data.input.pressed_keys.contains(&KeyCode::KeyA) {
            data.camera.position -= right * velocity;
        }
        let right = data.camera.right;
        if data.input.pressed_keys.contains(&KeyCode::KeyD) {
            data.camera.position += right * velocity;
        }

        // Update camera pitch
        let mut xoffset = data.input.mouse_delta.x as f32;
        let mut yoffset = data.input.mouse_delta.y as f32;
        let constrain_pitch = true; // Assuming this is always true for now, or could be a resource/config

        xoffset *= data.camera.mouse_sensitivity;
        yoffset *= data.camera.mouse_sensitivity;

        data.camera.yaw += xoffset;
        data.camera.pitch -= yoffset;

        if constrain_pitch {
            if data.camera.pitch > 89.0 {
                data.camera.pitch = 89.0;
            }
            if data.camera.pitch < -89.0 {
                data.camera.pitch = -89.0;
            }
        }

        // Call the internal update_camera_vectors logic
        let yaw_radians = data.camera.yaw.to_radians();
        let pitch_radians = data.camera.pitch.to_radians();

        let x = yaw_radians.cos() * pitch_radians.cos();
        let y = pitch_radians.sin();
        let z = yaw_radians.sin() * pitch_radians.cos();

        data.camera.front = Vec3::new(x, y, z).normalize();
        data.camera.right = data.camera.front.cross(data.camera.world_up).normalize();
        data.camera.up = data.camera.right.cross(data.camera.front).normalize();

        // Handle scrolling
        let yoffset_scroll = data.input.scroll_delta.y;
        data.camera.zoom -= yoffset_scroll;
        if data.camera.zoom < 1.0 {
            data.camera.zoom = 1.0;
        }
        if data.camera.zoom > 45.0 {
            data.camera.zoom = 45.0;
        }
        data.camera.projection_dirty = true;

        // Update math
        data.camera.view_matrix = Mat4::look_at_rh(
            data.camera.position,
            data.camera.position + data.camera.front,
            data.camera.up,
        );

        // (projection matrix is expensive, only update if dirty)
        if data.camera.projection_dirty {
            data.camera.projection_matrix = Mat4::perspective_rh_gl(
                data.camera.zoom.to_radians(),
                data.window.aspect_ratio(),
                0.1,
                100.0,
            );
            data.camera.projection_dirty = false;
        }
    }
}
