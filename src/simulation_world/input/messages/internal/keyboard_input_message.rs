use bevy_ecs::prelude::Message;
use winit::{event::ElementState, keyboard::PhysicalKey};

#[derive(Message, Debug, Clone)]
pub struct KeyboardInputMessage {
    pub key_code: PhysicalKey,
    pub state: ElementState,
}
