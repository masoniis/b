use bevy_ecs::prelude::*;
use winit::window::CursorGrabMode;

#[derive(Debug, Resource)]
pub struct DesiredCursorState {
    pub visible: bool,
    pub grab_mode: CursorGrabMode,
}

impl Default for DesiredCursorState {
    fn default() -> Self {
        Self {
            visible: true,
            grab_mode: CursorGrabMode::None,
        }
    }
}
