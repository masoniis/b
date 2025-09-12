use glam::{DVec2, Vec2};
use shred::World;
use winit::event::{DeviceEvent, ElementState, WindowEvent};
use winit::keyboard::PhysicalKey;

use crate::ecs::resources::input::InputResource;

/// The input system is responsible for handling all forms of input.
pub struct InputSystem;

impl InputSystem {
    /// Resets per-frame input state.
    pub fn new_events_hook(&mut self, world: &mut World) {
        let mut input = world.fetch_mut::<InputResource>();
        input.mouse_delta = DVec2::ZERO;
        input.scroll_delta = Vec2::ZERO;
    }

    /// Processes window-specific events, like keyboard input.
    pub fn window_event_hook(&mut self, world: &mut World, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput {
            event: key_event, ..
        } = event
        {
            let mut input = world.fetch_mut::<InputResource>();
            match key_event.state {
                ElementState::Pressed => {
                    if let PhysicalKey::Code(key_code) = key_event.physical_key {
                        input.pressed_keys.insert(key_code);
                    }
                }
                ElementState::Released => {
                    if let PhysicalKey::Code(key_code) = key_event.physical_key {
                        input.pressed_keys.remove(&key_code);
                    }
                }
            }
        }
    }

    /// Processes device-agnostic events, like raw mouse motion.
    pub fn device_event_hook(&mut self, world: &mut World, event: &DeviceEvent) {
        let mut input = world.fetch_mut::<InputResource>();
        match event {
            DeviceEvent::MouseMotion { delta } => {
                input.mouse_delta.x += delta.0;
                input.mouse_delta.y += delta.1;
            }
            DeviceEvent::MouseWheel { delta, .. } => {
                let yoffset = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                input.scroll_delta.y += yoffset;
            }
            _ => (),
        }
    }
}
