use bevy_ecs::event::Event;

#[derive(Event, Clone, Copy)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}
