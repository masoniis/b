use bevy_ecs::prelude::Message;
use glam::DVec2;

#[derive(Message, Debug, Clone)]
pub struct MouseMoveMessage {
    pub delta: DVec2,
}
