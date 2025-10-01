use crate::{
    ecs_modules::input::{
        events::{KeyboardInputEvent, MouseButtonInputEvent, RawWindowEvent},
        InputResource,
    },
    ecs_resources::WindowResource,
};
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::ResMut,
};
use winit::event::{ElementState, WindowEvent};

/// A system to handle external raw window events from the OS (via winit),
/// and convert them into ECS world events (as well as updating input state)
pub fn window_events_system(
    // State to modify
    mut input_resource: ResMut<InputResource>,
    mut window_resource: ResMut<WindowResource>,

    // Input from OS bridge
    mut raw_window_events: EventReader<RawWindowEvent>,

    // Output
    mut keyboard_writer: EventWriter<KeyboardInputEvent>,
    mut mouse_button_writer: EventWriter<MouseButtonInputEvent>,
) {
    input_resource.swap_previous_and_reset_deltas();

    // INFO: ----------------------------------
    //         Handle raw window events
    // ----------------------------------------

    for RawWindowEvent(event) in raw_window_events.read() {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let semantic_event = KeyboardInputEvent {
                    key_code: event.physical_key,
                    state: event.state,
                };

                match semantic_event.state {
                    ElementState::Pressed => input_resource.key_press(semantic_event.key_code),
                    ElementState::Released => input_resource.key_release(semantic_event.key_code),
                }

                keyboard_writer.write(semantic_event);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                let semantic_event = MouseButtonInputEvent {
                    button: *button,
                    state: *state,
                };

                match semantic_event.state {
                    ElementState::Pressed => input_resource.mouse_press(semantic_event.button),
                    ElementState::Released => input_resource.mouse_release(semantic_event.button),
                }

                mouse_button_writer.write(semantic_event);
            }
            WindowEvent::Resized(physical_size) => {
                window_resource.width = physical_size.width;
                window_resource.height = physical_size.height;
            }
            _ => {}
        }
    }
}
