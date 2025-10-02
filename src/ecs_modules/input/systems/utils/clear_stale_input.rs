use crate::ecs_modules::input::events::RawDeviceEvent;
use crate::prelude::*;
use bevy_ecs::prelude::*;

/// Clears any input events that accumulated, likely during the loading screen or something.
pub fn clear_stale_input_events_system(mut device_events: ResMut<Events<RawDeviceEvent>>) {
    info!("Clearing stale input events...");
    device_events.clear();
}
