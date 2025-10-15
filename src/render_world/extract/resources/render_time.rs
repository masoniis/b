use crate::{
    render_world::extract::extract_resource::ExtractResource,
    simulation_world::global_resources::time::WorldTimeResource,
};
use bevy_ecs::{
    prelude::Resource,
    system::{Commands, ResMut},
};

#[derive(Resource, Debug, Default)]
pub struct RenderTimeResource {
    pub total_elapsed_seconds: f32,
}

impl ExtractResource for RenderTimeResource {
    type Source = WorldTimeResource;
    type Output = RenderTimeResource;

    /// Extracts the time resource. Because time always changes, this performs
    /// an unconditional update every frame.
    fn extract_and_update(
        commands: &mut Commands,
        source: &Self::Source,
        _target: Option<ResMut<Self::Output>>,
    ) {
        // Since elapsed time always changed we can just insert it and
        // trigger updates every frame, no point in doing conditional change checking
        commands.insert_resource(RenderTimeResource {
            total_elapsed_seconds: source.total_elapse.as_secs_f32(),
        });
    }
}
