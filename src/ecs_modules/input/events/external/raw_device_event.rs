use bevy_ecs::prelude::Event;
use winit::event::DeviceEvent;

// NOTE: winit events aren't always Clone(able)
#[derive(Event, Debug, Clone)]
pub struct RawDeviceEvent(pub DeviceEvent);
