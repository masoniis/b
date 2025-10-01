use bevy_ecs::event::Event;
use winit::event::ElementState;
use winit::keyboard::PhysicalKey;

#[derive(Event, Debug, Clone)]
pub struct KeyboardInputEvent {
    pub key_code: PhysicalKey,
    pub state: ElementState,
}
