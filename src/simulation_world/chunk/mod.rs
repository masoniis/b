pub mod break_voxel;
pub mod components;
pub mod consts;
pub mod meshing;
pub mod tasks;
pub mod types;

pub use break_voxel::*;
pub use components::*;
pub use consts::*;
pub use meshing::*;
pub use tasks::*;
pub use types::*;

// INFO: ------------------------------
//         Chunk loading plugin
// ------------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{camera::ActiveCamera, scheduling::FixedUpdateSet, SimulationSchedule},
    SimulationSet,
};
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::{message::Messages, prelude::*};

pub struct ChunkLoadingPlugin;

impl Plugin for ChunkLoadingPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(ChunkStateManager::default());
        builder.init_resource::<Messages<BreakVoxelEvent>>();

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (manage_distance_based_chunk_loading_targets_system)
                    .run_if(
                        |camera: Res<ActiveCamera>, q: Query<(), Changed<ChunkCoord>>| {
                            q.get(camera.0).is_ok()
                        },
                    )
                    .in_set(SimulationSet::PreUpdate),
            )
            .add_systems((break_voxel_system,).in_set(SimulationSet::Update));

        builder
            .schedule_entry(SimulationSchedule::FixedUpdate)
            .add_systems(
                (
                    handle_dirty_chunks_system,
                    start_pending_generation_tasks_system,
                    poll_chunk_generation_tasks,
                    start_pending_meshing_tasks_system,
                    poll_chunk_meshing_tasks,
                )
                    .in_set(FixedUpdateSet::MainLogic),
            );
    }
}
