use bevy_ecs::event::Event;
use winit::event::{ElementState, MouseButton};

#[derive(Event, Debug, Clone)]
pub struct MouseButtonInputEvent {
    pub button: MouseButton,
    pub state: ElementState,
}
