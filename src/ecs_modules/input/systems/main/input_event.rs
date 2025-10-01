use crate::ecs_modules::input::{
    events::{
        KeyboardInputEvent, MouseButtonInputEvent, MouseMoveEvent, MouseScrollEvent,
        RawDeviceEvent, RawWindowEvent,
    },
    InputResource,
};
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::ResMut,
};
use winit::event::{ElementState, WindowEvent};

/// A system to handle external raw input events from the OS (via winit),
/// and simultaneously update the input resource with device information.
pub fn input_event_system(
    // State to modify
    mut input_resource: ResMut<InputResource>,

    // Input from OS bridge
    mut raw_window_events: EventReader<RawWindowEvent>,
    mut raw_device_events: EventReader<RawDeviceEvent>,

    // Output
    mut keyboard_writer: EventWriter<KeyboardInputEvent>,
    mut mouse_button_writer: EventWriter<MouseButtonInputEvent>,
    mut mouse_move_writer: EventWriter<MouseMoveEvent>,
    mut mouse_scroll_writer: EventWriter<MouseScrollEvent>,
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
            _ => {}
        }
    }

    // INFO: ----------------------------------
    //         Handle raw device events
    // ----------------------------------------

    for RawDeviceEvent(event) in raw_device_events.read() {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                let semantic_event = MouseMoveEvent {
                    delta: (*delta).into(),
                };

                input_resource.adjust_mouse_delta(semantic_event.delta);

                mouse_move_writer.write(semantic_event);
            }
            winit::event::DeviceEvent::MouseWheel { delta, .. } => {
                let yoffset = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                let semantic_event = MouseScrollEvent {
                    delta: glam::Vec2::new(0.0, yoffset),
                };

                input_resource.adjust_scroll_delta(semantic_event.delta);

                mouse_scroll_writer.write(semantic_event);
            }
            _ => {}
        }
    }
}
