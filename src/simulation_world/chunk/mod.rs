pub mod components;
pub mod consts;
pub mod generation;
pub mod management;
pub mod meshing;

pub use components::*;
pub use consts::*;
pub use generation::*;
pub use management::*;
pub use meshing::*;

// INFO: --------------------------
//         Chunk gen plugin
// --------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        chunk::{
            async_chunking::{
                poll_chunk_generation_tasks, poll_chunk_meshing_tasks,
                start_pending_generation_tasks_system, start_pending_meshing_tasks_system,
            },
            chunk_loader::manage_chunk_loading_system,
            core::ActiveBiomeGenerator,
            load_manager::ChunkLoadManager,
        },
        SimulationSchedule,
    },
    SimulationSet,
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_resource(ChunkLoadManager::default())
            .add_resource(ActiveBiomeGenerator::default())
            .add_resource(ActiveChunkGenerator::default());

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (
                    manage_chunk_loading_system,
                    start_pending_generation_tasks_system,
                    poll_chunk_generation_tasks,
                    start_pending_meshing_tasks_system,
                    poll_chunk_meshing_tasks,
                )
                    .in_set(SimulationSet::Update),
            );
    }
}
