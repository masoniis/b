pub mod components;
pub mod generators;
pub mod public;
pub mod systems;

pub use components::*;
pub use generators::*;
pub use public::*;

// INFO: ----------------------------
//         Terrain gen plugin
// ----------------------------------

use crate::prelude::*;
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::input::ActionStateResource,
};
use bevy_ecs::prelude::{IntoScheduleConfigs, Res};
use systems::cycle_active_generator;

pub struct TerrainGenerationPlugin;

impl Plugin for TerrainGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_resource(ClimateNoiseGenerator::new(0)) // hardcode seed 0 for now
            .add_resource(ActiveClimateGenerator::default())
            .add_resource(ActiveBiomeGenerator::default())
            .add_resource(ActiveTerrainGenerator::default())
            .add_resource(ActiveTerrainPainter::default());

        // INFO: -----------------------
        //         startup stuff
        // -----------------------------
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(cycle_active_generator.in_set(StartupSet::ResourceInitialization)); // set generator based on cycler at startup to override the default

        // INFO: -------------------------------------
        //         keybind-based actions below
        // -------------------------------------------

        // set desired cursor state on pause action
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(cycle_active_generator.run_if(
                |action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::CycleActiveTerrainGenerator)
                },
            ));
    }
}
