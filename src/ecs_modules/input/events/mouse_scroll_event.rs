use bevy_ecs::event::Event;
use glam::Vec2;

#[derive(Event, Debug, Clone)]
pub struct MouseScrollEvent {
    pub delta: Vec2,
}
