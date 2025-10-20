use bevy_ecs::prelude::Message;

#[derive(Message, Clone, Copy)]
pub struct MouseResizeMessage {
    pub width: u32,
    pub height: u32,
}
