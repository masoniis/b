use crate::ecs_resources::input::InputResource;
use bevy_ecs::system::ResMut;

pub fn update_input_system(mut input_resource: ResMut<InputResource>) {
    input_resource.previous_keys = input_resource.current_keys.clone();
    input_resource.mouse_delta = glam::DVec2::ZERO;
    input_resource.scroll_delta = glam::Vec2::ZERO;
}
