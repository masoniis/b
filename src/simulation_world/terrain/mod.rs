pub mod components;
pub mod generators;
pub mod public;
pub mod systems;

pub use components::*;
pub use generators::*;
pub use public::*;

// INFO: ----------------------------
//         terrain gen plugin
// ----------------------------------

use crate::prelude::*;
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::input::ActionStateResource,
};
use bevy_ecs::prelude::{IntoScheduleConfigs, Res};
pub use systems::{cycle_active_generator, TerrainGeneratorLibrary};

pub struct TerrainGenerationPlugin;

impl Plugin for TerrainGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_resource(ClimateNoiseGenerator::new(0)) // hardcode seed 0 for presentation reproducibility
            .add_resource(ActiveClimateGenerator::default())
            .add_resource(ActiveBiomeGenerator::default())
            .add_resource(ActiveTerrainGenerator::default())
            .add_resource(ActiveTerrainPainter::default())
            .init_resource::<TerrainGeneratorLibrary>();

        // INFO: -------------------------------
        //         keybind-based actions
        // -------------------------------------

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(cycle_active_generator.run_if(
                |action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::CycleActiveTerrainGenerator)
                },
            ));
    }
}
