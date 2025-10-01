use crate::ecs_modules::input::{
    events::{MouseMoveEvent, MouseScrollEvent, RawDeviceEvent},
    InputResource,
};
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::ResMut,
};

/// A system to handle external raw input events from the OS (via winit),
/// and simultaneously update the input resource with device information.
pub fn device_events_system(
    // State to modify
    mut input_resource: ResMut<InputResource>,

    // Input from OS bridge
    mut raw_device_events: EventReader<RawDeviceEvent>,

    // Output
    mut mouse_move_writer: EventWriter<MouseMoveEvent>,
    mut mouse_scroll_writer: EventWriter<MouseScrollEvent>,
) {
    input_resource.swap_previous_and_reset_deltas();

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
