use crate::game_world::{
    global_resources::CameraResource,
    input::{
        events::{
            internal::WindowResizeEvent, KeyboardInputEvent, MouseButtonInputEvent, RawWindowEvent,
        },
        resources::{Buttons, WindowSizeResource},
    },
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

    // Input from OS bridge
    mut raw_window_events: EventReader<RawWindowEvent>,

    // Output
    mut keyboard_writer: EventWriter<KeyboardInputEvent>,
    mut mouse_button_writer: EventWriter<MouseButtonInputEvent>,
    mut resize_writer: EventWriter<WindowResizeEvent>,
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
                resize_writer.write(WindowResizeEvent {
                    width: physical_size.width,
                    height: physical_size.height,
                });
            }
            _ => {}
        }
    }
}

/// A system that listens for `WindowResizedEvent`s and updates relevant resources.
pub fn handle_resize_system(
    mut resize_events: EventReader<WindowResizeEvent>,
    mut window_resource: ResMut<WindowSizeResource>,
    mut camera: ResMut<CameraResource>,
) {
    for event in resize_events.read() {
        window_resource.width = event.width;
        window_resource.height = event.height;
        camera.projection_dirty = true;
    }
}
