use bevy_ecs::event::Event;
use glam::DVec2;

#[derive(Event, Debug, Clone)]
pub struct MouseMoveEvent {
    pub delta: DVec2,
}
