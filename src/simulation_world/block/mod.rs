pub mod block_definition;
pub mod block_registry;
pub mod selected_block;

pub use block_definition::{load_block_from_str, BlockFaceTextures, BlockProperties};
pub use block_registry::{BlockId, BlockRegistryResource, AIR_BLOCK_ID, SOLID_BLOCK_ID};
pub use selected_block::TargetedBlock;

// INFO: ----------------------
//         Block plugin
// ----------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        block::block_registry::initialize_block_registry_system, scheduling::StartupSet,
        SimulationSchedule,
    },
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(BlockRegistryResource::default());
        builder.add_resource(TargetedBlock::default());

        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(
                initialize_block_registry_system.in_set(StartupSet::ResourceInitialization),
            );
    }
}
