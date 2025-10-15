pub mod frame_clock;
pub mod simulation_tick;
pub mod world_clock;

pub use frame_clock::FrameClock;
pub use world_clock::WorldClockResource;

// INFO: ---------------------
//         Time plugin
// ---------------------------

use crate::simulation_world::time::frame_clock::update_frame_clock_system;
use crate::simulation_world::time::world_clock::update_world_clock_system;
use crate::{
    ecs_core::{in_state, EcsBuilder, Plugin},
    simulation_world::{app_lifecycle::AppState, SimulationSchedule, SimulationSet},
};
use bevy_ecs::prelude::*;

pub struct TimeControlPlugin;

impl Plugin for TimeControlPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // Maintain a clock that tracks frame time and provides timing info
        builder
            // resources
            .add_resource(FrameClock::default())
            // systems
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (update_frame_clock_system)
                    .in_set(SimulationSet::PreUpdate)
                    .run_if(in_state(AppState::Running)),
            );

        builder
            .add_resource(WorldClockResource::default())
            .schedule_entry(SimulationSchedule::FixedUpdate)
            .add_systems(update_world_clock_system);
    }
}
