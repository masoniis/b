use bevy_ecs::prelude::Event;
use winit::event::DeviceEvent;

#[derive(Event, Debug)]
pub struct RawDeviceEvent(pub DeviceEvent);
