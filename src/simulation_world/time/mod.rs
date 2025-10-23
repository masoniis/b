pub mod frame_clock;
pub mod simulation_tick;
pub mod world_clock;

pub use frame_clock::FrameClock;
pub use world_clock::WorldClockResource;

// INFO: ---------------------
//         Time plugin
// ---------------------------

use crate::ecs_core::state_machine::AppState;
use crate::simulation_world::time::frame_clock::update_frame_clock_system;
use crate::simulation_world::time::simulation_tick::{run_fixed_update_schedule, SimulationTick};
use crate::simulation_world::time::world_clock::update_world_clock_system;
use crate::{
    ecs_core::{state_machine::utils::in_state, EcsBuilder, Plugin},
    simulation_world::{SimulationSchedule, SimulationSet},
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

        // Trigger the simulation ticks when appropriate
        builder
            .add_resource(SimulationTick::default())
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                run_fixed_update_schedule
                    .in_set(SimulationSet::Update)
                    .run_if(in_state(AppState::Running)),
            );

        // Maintain world clock that depends on ticks rather that frames
        builder
            .add_resource(WorldClockResource::default())
            .schedule_entry(SimulationSchedule::FixedUpdate)
            .add_systems(update_world_clock_system);
    }
}
