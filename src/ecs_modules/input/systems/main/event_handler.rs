use crate::ecs_resources::{
    events::{KeyboardInputEvent, MouseInputEvent, MouseScrollEvent},
    input::InputResource,
};
use bevy_ecs::{event::EventReader, system::ResMut};
use winit::event::ElementState;

pub fn input_event_handler(
    mut keyboard_input_events: EventReader<KeyboardInputEvent>,
    mut mouse_input_events: EventReader<MouseInputEvent>,
    mut mouse_scroll_events: EventReader<MouseScrollEvent>,
    mut input_resource: ResMut<InputResource>,
) {
    for event in keyboard_input_events.read() {
        match event.state {
            ElementState::Pressed => {
                input_resource.current_keys.insert(event.key_code.clone());
            }
            ElementState::Released => {
                input_resource.current_keys.remove(&event.key_code);
            }
        }
    }

    for event in mouse_input_events.read() {
        input_resource.mouse_delta += event.delta;
    }

    for event in mouse_scroll_events.read() {
        input_resource.scroll_delta += event.delta;
    }
}
