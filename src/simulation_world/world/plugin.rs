use super::systems::{main as main_system, startup as startup_system};
use crate::{
    ecs_core::{in_state, EcsBuilder, Plugin},
    simulation_world::{app_lifecycle::AppState, SimulationSchedule, SimulationSet},
};
use bevy_ecs::prelude::*;

pub struct WorldModulePlugin;

impl Plugin for WorldModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems((
                // startup_system::chunk_generation_system,
                startup_system::cube_array_generation_system,
            ));

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (main_system::time_system,)
                    .in_set(SimulationSet::PreUpdate)
                    .run_if(in_state(AppState::Running)),
            );
    }
}
