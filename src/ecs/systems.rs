use crate::ecs::world::World;
use crate::graphics::camera::CameraMovement;
use winit::event::{DeviceEvent, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

pub trait System {
    /// A hook that enables the system to perform actions BEFORE any events
    /// are processed. Useful for once-per frame actions like clock updates.
    fn new_events_hook(&mut self, _world: &mut World) {}
    /// A hook that enables a system to take action in response to window events.
    fn window_event_hook(&mut self, _world: &mut World, _event: &WindowEvent, _window: &Window) {}
    /// A hook that enables a system to take action in response to device events.
    fn device_event_hook(&mut self, _world: &mut World, _event: &DeviceEvent) {}
}

/// The input system is responsible for handling all forms of input from the
/// operating system. This primarily encompasses mouse and keyboard events.
pub struct InputSystem;
impl System for InputSystem {
    fn window_event_hook(&mut self, world: &mut World, event: &WindowEvent, _window: &Window) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => self.handle_keyboard_input(world, event),
            _ => (),
        }
    }

    fn device_event_hook(&mut self, world: &mut World, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => self.handle_mouse_motion(world, *delta),
            DeviceEvent::MouseWheel { delta, .. } => self.handle_mouse_scroll(world, delta),
            _ => (),
        }
    }
}
impl InputSystem {
    fn handle_keyboard_input(&mut self, world: &mut World, key_event: &KeyEvent) {
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
    fn handle_mouse_motion(&mut self, world: &mut World, delta: (f64, f64)) {
        world
            .camera
            .process_mouse_movement(delta.0 as f32, -(delta.1 as f32), true);
    }
    fn handle_mouse_scroll(&mut self, world: &mut World, delta: &winit::event::MouseScrollDelta) {
        let yoffset = match delta {
            winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
            winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
        };
        world.camera.process_mouse_scroll(yoffset);
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
