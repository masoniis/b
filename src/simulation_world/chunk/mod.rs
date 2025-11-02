pub mod components;
pub mod consts;
pub mod loading;

pub use components::*;
pub use consts::*;
pub use loading::*;

// INFO: ------------------------------
//         Chunk loading plugin
// ------------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{camera::ActiveCamera, scheduling::FixedUpdateSet, SimulationSchedule},
    SimulationSet,
};
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct ChunkLoadingPlugin;

impl Plugin for ChunkLoadingPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(ChunkLoadingManager::default());

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (manage_chunk_loading_system, manage_chunk_meshing_system)
                    .run_if(
                        |camera: Res<ActiveCamera>, q: Query<(), Changed<ChunkCoord>>| {
                            q.get(camera.0).is_ok()
                        },
                    )
                    .in_set(SimulationSet::Update),
            );

        builder
            .schedule_entry(SimulationSchedule::FixedUpdate)
            .add_systems(
                (
                    start_pending_generation_tasks_system,
                    poll_chunk_generation_tasks,
                    start_pending_meshing_tasks_system,
                    poll_chunk_meshing_tasks,
                )
                    .in_set(FixedUpdateSet::MainLogic),
            );
    }
}
