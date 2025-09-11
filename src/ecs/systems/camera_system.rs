use crate::ecs::resources::camera::CameraMovement;
use crate::ecs::systems::System;
use crate::ecs::world::World;
use winit::keyboard::KeyCode;

pub struct CameraSystem;

impl System for CameraSystem {
    fn new_events_hook(&mut self, world: &mut World) {
        // Camera movement logic
        if world.input_resource.pressed_keys.contains(&KeyCode::KeyW) {
            world
                .camera
                .process_keyboard(CameraMovement::Forward, world.delta_time.0);
        }
        if world.input_resource.pressed_keys.contains(&KeyCode::KeyS) {
            world
                .camera
                .process_keyboard(CameraMovement::Backward, world.delta_time.0);
        }
        if world.input_resource.pressed_keys.contains(&KeyCode::KeyA) {
            world
                .camera
                .process_keyboard(CameraMovement::Left, world.delta_time.0);
        }
        if world.input_resource.pressed_keys.contains(&KeyCode::KeyD) {
            world
                .camera
                .process_keyboard(CameraMovement::Right, world.delta_time.0);
        }

        // Camera math updates
        world.camera.update_view_matrix();
        if world.camera.projection_dirty {
            let aspect_ratio = world.window_size.0 as f32 / world.window_size.1 as f32;
            world.camera.update_projection_matrix(aspect_ratio);
            world.camera.projection_dirty = false;
        }
    }
}
