use crate::prelude::*;
use crate::simulation_world::time::simulation_tick::SimulationTick;
use bevy_ecs::prelude::*;
use std::time::Duration;

pub const SECONDS_IN_A_DAY: f32 = 1200.0;

/// A resource that tracks the in-game date and time.
#[derive(Resource, Debug)]
pub struct WorldClockResource {
    /// The total number of full days that have passed since the world began.
    pub total_days: u64,
    /// The current time within the 24-hour cycle.
    pub time_of_day: Duration,
    /// The duration of a full in-game day.
    pub day_duration: Duration,
}

impl Default for WorldClockResource {
    fn default() -> Self {
        Self {
            total_days: 0,
            time_of_day: Duration::from_secs_f32(SECONDS_IN_A_DAY * 0.25),
            day_duration: Duration::from_secs_f32(SECONDS_IN_A_DAY),
        }
    }
}

impl WorldClockResource {
    /// Returns the current time of day as a value from 0.0 (midnight) to 1.0 (next midnight).
    pub fn day_night_cycle_value(&self) -> f32 {
        self.time_of_day.as_secs_f32() / SECONDS_IN_A_DAY
    }
}

// INFO: ----------------------------
//         Update world clock
// ----------------------------------

/// A system that runs every tick to advance the in-game calendar.
#[instrument(skip_all)]
pub fn update_world_clock_system(
    // Input
    sim_tick: Res<SimulationTick>,

    // Output
    mut world_clock: ResMut<WorldClockResource>,
) {
    world_clock.time_of_day += sim_tick.tick_duration;

    if world_clock.time_of_day.as_secs_f32() >= SECONDS_IN_A_DAY {
        world_clock.time_of_day -= Duration::from_secs_f32(SECONDS_IN_A_DAY);
        world_clock.total_days += 1;
    }
}
