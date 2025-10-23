pub mod components;
pub mod consts;
pub mod generation;
pub mod management;
pub mod meshing;
pub mod types;

pub use components::*;
pub use consts::*;
pub use generation::*;
pub use management::*;
pub use meshing::*;
pub use types::*;

// INFO: --------------------------
//         Chunk gen plugin
// --------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        chunk::{
            async_chunking::poll_chunk_generation_tasks, chunk_loader::manage_chunk_loading_system,
            load_manager::ChunkLoadManager,
        },
        SimulationSchedule,
    },
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(ChunkLoadManager::default());

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (
                    manage_chunk_loading_system,
                    poll_chunk_generation_tasks,
                    chunk_meshing_system,
                )
                    .chain(),
            );
    }
}
