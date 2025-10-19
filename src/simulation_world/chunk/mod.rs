pub mod components;
pub mod generation;
pub mod meshing;
pub mod types;

pub use components::*;
pub use generation::*;
pub use meshing::*;
pub use types::*;

// INFO: --------------------------
//         Chunk gen plugin
// --------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        block::load_block_definitions_system, chunk::flat_world::setup_superflat_world,
        scheduling::StartupSet, SimulationSchedule,
    },
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(
                setup_superflat_world
                    .after(load_block_definitions_system)
                    .in_set(StartupSet::LoadingTasks),
            );

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(chunk_meshing_system);
    }
}
