use crate::ecs::resources::{
    camera::CameraMovement, input::InputResource, Camera, TimeResource, WindowResource,
};
use shred::{Read, System, SystemData, Write};
use winit::keyboard::KeyCode;

#[derive(SystemData)]
pub struct CameraSystemData<'a> {
    input: Read<'a, InputResource>,
    time: Read<'a, TimeResource>,
    window: Read<'a, WindowResource>,
    camera: Write<'a, Camera>,
}

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = CameraSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // Camera movement
        if data.input.pressed_keys.contains(&KeyCode::KeyW) {
            data.camera
                .process_keyboard(CameraMovement::Forward, data.time.delta_seconds);
        }
        if data.input.pressed_keys.contains(&KeyCode::KeyS) {
            data.camera
                .process_keyboard(CameraMovement::Backward, data.time.delta_seconds);
        }
        if data.input.pressed_keys.contains(&KeyCode::KeyA) {
            data.camera
                .process_keyboard(CameraMovement::Left, data.time.delta_seconds);
        }
        if data.input.pressed_keys.contains(&KeyCode::KeyD) {
            data.camera
                .process_keyboard(CameraMovement::Right, data.time.delta_seconds);
        }

        // Camera rotation
        data.camera.process_mouse_movement(
            data.input.mouse_delta.x as f32,
            -data.input.mouse_delta.y as f32,
            true,
        );

        // Camera zoom (only vertical)
        data.camera.process_mouse_scroll(data.input.scroll_delta.y);

        // Update the camera math AFTER all the above is decided
        data.camera.update_view_matrix();
        if data.camera.projection_dirty {
            data.camera
                .update_projection_matrix(data.window.aspect_ratio());
            data.camera.projection_dirty = false;
        }
    }
}
