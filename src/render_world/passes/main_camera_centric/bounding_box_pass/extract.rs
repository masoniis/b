use crate::{
    render_world::global_extract::extract_resource::ExtractResource,
    simulation_world::input::systems::toggle_chunk_borders::ChunkBoundsToggle,
};
use bevy_ecs::{
    resource::Resource,
    system::{Commands, ResMut},
};

#[derive(Resource, Debug, PartialEq, Eq)]
pub struct WireframeToggleState {
    pub enabled: bool,
}

pub struct WireframeToggleExtractor;

/// Extract the current chunk border toggle from the simulation world
impl ExtractResource for WireframeToggleExtractor {
    type Source = ChunkBoundsToggle;
    type Output = WireframeToggleState;

    fn extract_and_update(
        commands: &mut Commands,
        source: &Self::Source,
        target: Option<ResMut<Self::Output>>,
    ) {
        let new_mode = WireframeToggleState {
            enabled: source.enabled,
        };

        if let Some(mut target) = target {
            if *target != new_mode {
                *target = new_mode;
            }
        } else {
            commands.insert_resource(new_mode);
        }
    }
}
