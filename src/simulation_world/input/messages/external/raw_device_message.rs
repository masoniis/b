use bevy_ecs::prelude::Message;
use winit::event::DeviceEvent;

#[derive(Message, Clone, Debug)]
pub struct RawDeviceMessage(pub DeviceEvent);
