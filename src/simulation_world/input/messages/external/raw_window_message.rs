use bevy_ecs::prelude::Message;
use winit::event::WindowEvent;

#[derive(Message, Clone, Debug)]
pub struct RawWindowMessage(pub WindowEvent);
