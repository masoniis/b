use bevy_ecs::prelude::Event;
use winit::event::WindowEvent;

#[derive(Event, Debug)]
pub struct RawWindowEvent(pub WindowEvent);
