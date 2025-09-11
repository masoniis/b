use crate::ecs::systems::System;
use crate::ecs::world::World;
use winit::event::{DeviceEvent, KeyEvent, WindowEvent};
use winit::keyboard::PhysicalKey;
use winit::window::Window;

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
