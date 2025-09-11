use crate::ecs::resources::Input;
use crate::graphics::camera::{Camera, CameraMovement};
use winit::event::WindowEvent;
use winit::keyboard::{KeyCode, PhysicalKey};

pub fn input_system(input: &mut Input, event: &WindowEvent) {
    if let WindowEvent::KeyboardInput {
        event: key_event, ..
    } = event
    {
        if let PhysicalKey::Code(key_code) = key_event.physical_key {
            match key_event.state {
                winit::event::ElementState::Pressed => {
                    input.pressed_keys.insert(key_code);
                }
                winit::event::ElementState::Released => {
                    input.pressed_keys.remove(&key_code);
                }
            }
        }
    }
}

pub fn camera_update_system(camera: &mut Camera, aspect_ratio: f32) {
    camera.update_view_matrix();
    camera.update_projection_matrix(aspect_ratio);
}

pub fn camera_movement_system(camera: &mut Camera, input: &Input, delta_time: f32) {
    if input.pressed_keys.contains(&KeyCode::KeyW) {
        camera.process_keyboard(CameraMovement::Forward, delta_time);
    }
    if input.pressed_keys.contains(&KeyCode::KeyS) {
        camera.process_keyboard(CameraMovement::Backward, delta_time);
    }
    if input.pressed_keys.contains(&KeyCode::KeyA) {
        camera.process_keyboard(CameraMovement::Left, delta_time);
    }
    if input.pressed_keys.contains(&KeyCode::KeyD) {
        camera.process_keyboard(CameraMovement::Right, delta_time);
    }
}
