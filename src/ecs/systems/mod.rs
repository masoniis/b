pub mod camera_movement_system;
pub mod camera_update_system;
pub mod input_system;
pub mod render_system;

use crate::ecs::world::World;
use winit::event::{DeviceEvent, WindowEvent};
use winit::window::Window;

pub trait System {
    /// A hook that enables the system to perform actions BEFORE any events
    /// are processed. Useful for once-per frame actions like clock updates.
    fn new_events_hook(&mut self, _world: &mut World) {}
    /// A hook that enables a system to take action in response to window events.
    fn window_event_hook(&mut self, _world: &mut World, _event: &WindowEvent, _window: &Window) {}
    /// A hook that enables a system to take action in response to device events.
    fn device_event_hook(&mut self, _world: &mut World, _event: &DeviceEvent) {}
}
