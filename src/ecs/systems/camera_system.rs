use crate::ecs::resources::{
    Camera, DeltaTimeResource, WindowResource, camera::CameraMovement, input::InputResource,
};
use shred::{Read, System, SystemData, Write};
use winit::keyboard::KeyCode;

#[derive(SystemData)]
pub struct CameraSystemData<'a> {
    input: Read<'a, InputResource>,
    delta_time: Read<'a, DeltaTimeResource>,
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
                .process_keyboard(CameraMovement::Forward, data.delta_time.seconds);
        }
        if data.input.pressed_keys.contains(&KeyCode::KeyS) {
            data.camera
                .process_keyboard(CameraMovement::Backward, data.delta_time.seconds);
        }
        if data.input.pressed_keys.contains(&KeyCode::KeyA) {
            data.camera
                .process_keyboard(CameraMovement::Left, data.delta_time.seconds);
        }
        if data.input.pressed_keys.contains(&KeyCode::KeyD) {
            data.camera
                .process_keyboard(CameraMovement::Right, data.delta_time.seconds);
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
