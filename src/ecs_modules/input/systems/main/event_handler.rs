use crate::ecs_modules::input::{
    events::{KeyboardInputEvent, MouseButtonInputEvent, MouseMoveEvent, MouseScrollEvent},
    InputResource,
};
use bevy_ecs::{event::EventReader, system::ResMut};
use winit::event::ElementState;

pub fn input_event_handler(
    mut keyboard_input_events: EventReader<KeyboardInputEvent>,
    mut mouse_input_events: EventReader<MouseMoveEvent>,
    mut mouse_scroll_events: EventReader<MouseScrollEvent>,
    mut mouse_button_input_events: EventReader<MouseButtonInputEvent>,
    mut input_resource: ResMut<InputResource>,
) {
    // INFO: --------------------------------
    //         Handle keyboard events
    // --------------------------------------

    for event in keyboard_input_events.read() {
        match event.state {
            ElementState::Pressed => {
                input_resource.key_press(event.key_code);
            }
            ElementState::Released => {
                input_resource.key_release(event.key_code);
            }
        }
    }

    // INFO: ------------------------------------
    //         Handle mouse button events
    // ------------------------------------------

    for event in mouse_button_input_events.read() {
        match event.state {
            ElementState::Pressed => {
                input_resource.mouse_press(event.button);
            }
            ElementState::Released => {
                input_resource.mouse_release(event.button);
            }
        }
    }

    // INFO: -------------------------------------------------
    //         Handle mouse delta events (scroll/move)
    // -------------------------------------------------------

    for event in mouse_input_events.read() {
        input_resource.adjust_mouse_delta(event.delta);
    }

    for event in mouse_scroll_events.read() {
        input_resource.adjust_scroll_delta(event.delta);
    }
}
