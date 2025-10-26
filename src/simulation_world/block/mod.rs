pub mod block_definition;
pub mod block_registry;

pub use block_definition::{load_block_from_str, BlockFaceTextures, BlockProperties};
pub use block_registry::{load_block_definitions_system, BlockRegistryResource};

// INFO: ----------------------
//         Block plugin
// ----------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{scheduling::StartupSet, SimulationSchedule},
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(BlockRegistryResource::default());

        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(load_block_definitions_system.in_set(StartupSet::Tasks));
    }
}
