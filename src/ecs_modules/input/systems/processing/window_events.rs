use crate::{
    ecs_modules::input::{
        events::{KeyboardInputEvent, MouseButtonInputEvent, RawWindowEvent},
        resources::Buttons,
    },
    ecs_resources::{graphics_context::GraphicsContextResource, window::WindowResource},
};
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::ResMut,
};
use winit::{
    event::{ElementState, MouseButton, WindowEvent},
    keyboard::PhysicalKey,
};

/// A system to handle external raw window events from the OS (via winit),
/// and convert them into ECS world events (as well as updating input state)
pub fn window_events_system(
    // State to modify
    mut keyboard_input: ResMut<Buttons<PhysicalKey>>,
    mut mouse_input: ResMut<Buttons<MouseButton>>,
    mut window_resource: ResMut<WindowResource>,
    mut gfx_resource: ResMut<GraphicsContextResource>,

    // Input from OS bridge
    mut raw_window_events: EventReader<RawWindowEvent>,

    // Output
    mut keyboard_writer: EventWriter<KeyboardInputEvent>,
    mut mouse_button_writer: EventWriter<MouseButtonInputEvent>,
) {
    // Clear previous stale state
    keyboard_input.swap_previous();
    mouse_input.swap_previous();

    for RawWindowEvent(event) in raw_window_events.read() {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let semantic_event = KeyboardInputEvent {
                    key_code: event.physical_key,
                    state: event.state,
                };

                match semantic_event.state {
                    ElementState::Pressed => keyboard_input.press(semantic_event.key_code),
                    ElementState::Released => keyboard_input.release(semantic_event.key_code),
                }

                keyboard_writer.write(semantic_event);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                let semantic_event = MouseButtonInputEvent {
                    button: *button,
                    state: *state,
                };

                match semantic_event.state {
                    ElementState::Pressed => mouse_input.press(semantic_event.button),
                    ElementState::Released => mouse_input.release(semantic_event.button),
                }

                mouse_button_writer.write(semantic_event);
            }
            WindowEvent::Resized(physical_size) => {
                window_resource.width = physical_size.width;
                window_resource.height = physical_size.height;
                gfx_resource.context.resize(*physical_size);
            }
            _ => {}
        }
    }
}
