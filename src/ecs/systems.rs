use crate::ecs::world::World;
use crate::graphics::camera::CameraMovement;
use winit::event::WindowEvent;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

pub trait System {
    /// A hook that enables the system to perform actions BEFORE any events
    /// are processed. Useful for once-per frame actions like clock updates.
    fn new_events_hook(&mut self, _world: &mut World) {}
    /// A hook that enables a system to take action in response to events.
    fn window_event_hook(&mut self, _world: &mut World, _event: &WindowEvent, _window: &Window) {}
}

pub struct InputSystem;
impl System for InputSystem {
    fn window_event_hook(&mut self, world: &mut World, event: &WindowEvent, _window: &Window) {
        if let WindowEvent::KeyboardInput {
            event: key_event, ..
        } = event
        {
            if let PhysicalKey::Code(key_code) = key_event.physical_key {
                match key_event.state {
                    winit::event::ElementState::Pressed => {
                        world.input_resource.pressed_keys.insert(key_code);
                    }
                    winit::event::ElementState::Released => {
                        world.input_resource.pressed_keys.remove(&key_code);
                    }
                }
            }
        }
    }
}

pub struct CameraUpdateSystem;
impl System for CameraUpdateSystem {
    fn new_events_hook(&mut self, world: &mut World) {
        let aspect_ratio = world.window_size.0 as f32 / world.window_size.1 as f32;
        world.camera.update_view_matrix();
        world.camera.update_projection_matrix(aspect_ratio);
    }
}

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

pub struct RenderSystem;
impl System for RenderSystem {
    fn window_event_hook(&mut self, world: &mut World, event: &WindowEvent, window: &Window) {
        if let WindowEvent::RedrawRequested = event {
            if let (Some(renderer), Some(shader_program)) = (&world.renderer, &world.shader_program)
            {
                renderer.set_frame(shader_program, &world.camera);
                shader_program.set_mat4("modelView", &world.camera.get_view_matrix());
                shader_program.set_mat4("projection", &world.camera.get_projection_matrix());
                window.request_redraw();
            }
        }
    }
}
