use crate::ecs::systems::System;
use crate::ecs::world::World;
use crate::graphics::camera::CameraMovement;
use winit::keyboard::KeyCode;

pub struct CameraMovementSystem;
impl System for CameraMovementSystem {
    fn new_events_hook(&mut self, world: &mut World) {
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
    }
}
