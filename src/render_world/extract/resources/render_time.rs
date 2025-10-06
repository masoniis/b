use crate::{
    game_world::global_resources::time::TimeResource,
    render_world::extract::extract_resource::ExtractResource,
};
use bevy_ecs::prelude::Resource;

#[derive(Resource, Debug, Default)]
pub struct RenderTimeResource {
    pub delta_seconds: f32,
}

impl ExtractResource for RenderTimeResource {
    type Source = TimeResource;
    type Output = Self;

    fn extract_resource(source: &Self::Source) -> Self::Output {
        RenderTimeResource {
            delta_seconds: source.total_elapse.as_secs_f32(),
        }
    }
}
