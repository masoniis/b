pub mod components;
pub mod consts;
pub mod management;
pub mod meshing;

pub use components::*;
pub use consts::*;
pub use management::*;
pub use meshing::*;

// INFO: ------------------------------
//         Chunk loading plugin
// ------------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{scheduling::FixedUpdateSet, SimulationSchedule},
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct ChunkLoadingPlugin;

impl Plugin for ChunkLoadingPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(ChunkLoadManager::default());

        builder
            .schedule_entry(SimulationSchedule::FixedUpdate)
            .add_systems(
                (
                    manage_chunk_loading_system,
                    start_pending_generation_tasks_system,
                    poll_chunk_generation_tasks,
                    start_pending_meshing_tasks_system,
                    poll_chunk_meshing_tasks,
                )
                    .in_set(FixedUpdateSet::MainLogic),
            );
    }
}
