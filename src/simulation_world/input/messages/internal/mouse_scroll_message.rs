use bevy_ecs::prelude::Message;
use glam::Vec2;

#[derive(Message, Debug, Clone)]
pub struct MouseScrollMessage {
    pub delta: Vec2,
}
