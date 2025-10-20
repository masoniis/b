use bevy_ecs::prelude::Message;
use winit::event::ElementState;
use winit::keyboard::PhysicalKey;

#[derive(Message, Debug, Clone)]
pub struct KeyboardInputMessage {
    pub key_code: PhysicalKey,
    pub state: ElementState,
}
