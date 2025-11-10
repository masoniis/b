pub mod components;
pub mod generators;
pub mod systems;

pub use generators::*;

use bevy_ecs::schedule::IntoScheduleConfigs;
pub use components::*;
pub use generators::core::{ActiveTerrainGenerator, ActiveTerrainPainter, TerrainShaper};
pub use systems::*;

use generators::core::ActiveBiomeGenerator;
use systems::cycle_active_generator::cycle_active_generator;

// INFO: ----------------------------
//         Terrain gen plugin
// ----------------------------------

use crate::prelude::*;
use crate::simulation_world::input::SimulationAction;
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::input::ActionStateResource,
};
use bevy_ecs::prelude::Res;

pub struct TerrainGenerationPlugin;

impl Plugin for TerrainGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_resource(ClimateNoiseGenerator::new(0)) // hardcode seed 0 for now
            .add_resource(ActiveBiomeGenerator::default())
            .add_resource(ActiveTerrainGenerator::default())
            .add_resource(ActiveTerrainPainter::default());

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
