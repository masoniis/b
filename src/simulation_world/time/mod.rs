pub mod world_time;

pub use world_time::time_system;

// INFO: ---------------------
//         Time plugin
// ---------------------------

use crate::{
    ecs_core::{in_state, EcsBuilder, Plugin},
    simulation_world::{app_lifecycle::AppState, SimulationSchedule, SimulationSet},
};
use bevy_ecs::prelude::*;

pub struct TimeControlPlugin;

impl Plugin for TimeControlPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (time_system,)
                    .in_set(SimulationSet::PreUpdate)
                    .run_if(in_state(AppState::Running)),
            );
    }
}
