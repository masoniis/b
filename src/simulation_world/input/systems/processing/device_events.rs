use crate::{
    prelude::*,
    simulation_world::input::{
        messages::{MouseMoveMessage, MouseScrollMessage, RawDeviceMessage},
        resources::CursorMovement,
    },
};
use bevy_ecs::{
    message::{MessageReader, MessageWriter},
    system::ResMut,
};

/// A system to handle external raw input events from the OS (via winit),
/// and simultaneously update the input resource with device information.
#[instrument(skip_all)]
pub fn device_events_system(
    // Input from OS bridge
    mut raw_device_events: MessageReader<RawDeviceMessage>,

    // State to modify (output)
    mut movement: ResMut<CursorMovement>,

    // Events to fire (output)
    mut mouse_move_writer: MessageWriter<MouseMoveMessage>,
    mut mouse_scroll_writer: MessageWriter<MouseScrollMessage>,
) {
    // Clear previous stale state (without this mouse movement would "accumulate")
    movement.reset_deltas();

    for RawDeviceMessage(event) in raw_device_events.read() {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                let semantic_event = MouseMoveMessage {
                    delta: (*delta).into(),
                };

                movement.adjust_mouse_delta(semantic_event.delta);

                mouse_move_writer.write(semantic_event);
            }
            winit::event::DeviceEvent::MouseWheel { delta, .. } => {
                let yoffset = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                let semantic_event = MouseScrollMessage {
                    delta: glam::Vec2::new(0.0, yoffset),
                };

                movement.adjust_scroll_delta(semantic_event.delta);

                mouse_scroll_writer.write(semantic_event);
            }
            _ => {}
        }
    }
}
