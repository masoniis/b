use bevy_ecs::event::Event;
use glam::{DVec2, Vec2};
use winit::{event::ElementState, keyboard::PhysicalKey};

#[derive(Event, Debug, Clone)]
pub struct KeyboardInputEvent {
    pub key_code: PhysicalKey,
    pub state: ElementState,
}

#[derive(Event, Debug, Clone)]
pub struct MouseInputEvent {
    pub delta: DVec2,
}

#[derive(Event, Debug, Clone)]
pub struct MouseScrollEvent {
    pub delta: Vec2,
}
