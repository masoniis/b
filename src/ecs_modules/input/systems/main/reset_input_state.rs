use crate::ecs_modules::input::InputResource;
use bevy_ecs::system::ResMut;

/// Reset any input state. Should be called at somepoint between the end of the
/// previous frame and the start of processing events for the current frame.
pub fn reset_input_state_system(mut input_resource: ResMut<InputResource>) {
    input_resource.previous_keys = input_resource.current_keys.clone();
    input_resource.mouse_delta = glam::DVec2::ZERO;
    input_resource.scroll_delta = glam::Vec2::ZERO;
}
