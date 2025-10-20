use bevy_ecs::prelude::Message;
use winit::event::{ElementState, MouseButton};

#[derive(Message, Debug, Clone)]
pub struct MouseButtonInputMessage {
    pub button: MouseButton,
    pub state: ElementState,
}
