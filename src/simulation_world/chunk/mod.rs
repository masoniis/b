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
        chunk::{chunk_spawner::manage_chunk_loading_system, load_manager::ChunkLoadManager},
        SimulationSchedule,
    },
};

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // builder
        //     .schedule_entry(SimulationSchedule::Startup)
        //     .add_systems(
        //         setup_superflat_world
        //             .after(load_block_definitions_system)
        //             .in_set(StartupSet::Tasks),
        //     );

        builder.add_resource(ChunkLoadManager::default());

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((chunk_meshing_system, manage_chunk_loading_system));
    }
}
