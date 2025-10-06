use crate::{
    game_world::input::resources::WindowSizeResource,
    render_world::extract::extract_resource::ExtractResource,
};
use bevy_ecs::prelude::Resource;

#[derive(Resource, Debug, Default)]
pub struct RenderWindowSizeResource {
    pub width: f32,
    pub height: f32,
}

impl ExtractResource for RenderWindowSizeResource {
    type Source = WindowSizeResource;
    type Output = Self;

    fn extract_resource(source: &Self::Source) -> Self::Output {
        RenderWindowSizeResource {
            width: source.width as f32,
            height: source.height as f32,
        }
    }
}
