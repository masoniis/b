use bevy_ecs::prelude::Event;
use winit::event::WindowEvent;

// NOTE: winit events aren't always Clone(able)
#[derive(Event, Debug, Clone)]
pub struct RawWindowEvent(pub WindowEvent);
