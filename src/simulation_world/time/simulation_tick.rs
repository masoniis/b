use crate::{
    prelude::*,
    simulation_world::{time::FrameClock, SimulationSchedule},
};
use bevy_ecs::prelude::*;
use std::time::Duration;

#[derive(Resource, Default, Debug)]
pub struct SimulationTick {
    pub tick: u64,

    pub tick_rate: f32,
    pub tick_duration: Duration,
}

/// Updates the simulation tick and clocks
#[instrument(skip_all)]
pub fn run_fixed_update_schedule(
    mut frame_clock: ResMut<FrameClock>,
    mut sim_tick: ResMut<SimulationTick>,
    world: &mut World,
) {
    while frame_clock.accumulator >= sim_tick.tick_duration {
        frame_clock.decrement_accumulator_tick(sim_tick.tick_duration);
        sim_tick.tick += 1;

        world.run_schedule(SimulationSchedule::FixedUpdate);
    }

    // Calculate alpha after processing fixed updates
    frame_clock.alpha =
        frame_clock.accumulator.as_secs_f32() / sim_tick.tick_duration.as_secs_f32();
}
