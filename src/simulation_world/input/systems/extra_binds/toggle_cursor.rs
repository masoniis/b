use crate::simulation_world::input::resources::DesiredCursorState;
use bevy_ecs::system::ResMut;

pub fn toggle_cursor_system(mut desired_cursor_state: ResMut<DesiredCursorState>) {
    desired_cursor_state.visible = !desired_cursor_state.visible;

    if desired_cursor_state.visible {
        desired_cursor_state.grab_mode = winit::window::CursorGrabMode::None;
    } else {
        desired_cursor_state.grab_mode = winit::window::CursorGrabMode::Locked;
    }
}
