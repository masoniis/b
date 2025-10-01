use crate::ecs_modules::input::events::{
    keyboard_input_event::KeyboardInputEvent, mouse_button_input_event::MouseButtonInputEvent,
    mouse_input_event::MouseMoveEvent, mouse_scroll_event::MouseScrollEvent,
};
use bevy_ecs::world::World;
use glam::Vec2;
use winit::event::{DeviceEvent, WindowEvent};

/// Handles window input and sends the events to the world for processing.
pub struct InputBridge;

impl InputBridge {
    /// Processes window-specific events, like keyboard input.
    pub fn window_event_hook(&mut self, world: &mut World, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                let key_code = key_event.physical_key;
                world.send_event(KeyboardInputEvent {
                    key_code,
                    state: key_event.state,
                });
            }
            WindowEvent::MouseInput { button, state, .. } => {
                world.send_event(MouseButtonInputEvent {
                    button: *button,
                    state: *state,
                });
            }
            _ => (),
        }
    }

    /// Processes device-agnostic events, like raw mouse motion.
    pub fn device_event_hook(&mut self, world: &mut World, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                world.send_event(MouseMoveEvent {
                    delta: (*delta).into(),
                });
            }
            DeviceEvent::MouseWheel { delta, .. } => {
                let yoffset = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                world.send_event(MouseScrollEvent {
                    delta: Vec2::new(0.0, yoffset),
                });
            }
            _ => (),
        }
    }
}
