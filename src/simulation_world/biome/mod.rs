pub mod biome_definition;
pub mod biome_registry;

pub use biome_definition::BiomeDefinition;
pub use biome_registry::{load_biome_definitions_system, BiomeRegistryResource};

// INFO: ----------------------
//         Biome plugin
// ----------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{scheduling::StartupSet, SimulationSchedule},
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct BiomePlugin;

impl Plugin for BiomePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(BiomeRegistryResource::default());

        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(load_biome_definitions_system.in_set(StartupSet::Tasks));
    }
}
