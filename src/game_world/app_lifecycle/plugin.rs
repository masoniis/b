use crate::{
    game_world::{
        schedules::GameSchedule,
        state_machine::{
            resources::{AppState, GameState},
            systems::apply_state_transition_system,
        },
        Plugin, ScheduleBuilder,
    },
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

use super::{finalize_loading_system, start_fake_work_system};

pub struct AppLifecyclePlugin;

impl Plugin for AppLifecyclePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, _world: &mut World) {
        // The state resources the state machine oversees
        schedules
            .entry(GameSchedule::Startup)
            .add_systems(start_fake_work_system);

        schedules.entry(GameSchedule::Loading).add_systems(
            (
                finalize_loading_system,
                apply_state_transition_system::<AppState>,
                crate::game_world::world::systems::main::time::time_system, // Add time_system
            )
                .chain(),
        );

        schedules.entry(GameSchedule::Main).add_systems(
            (
                apply_state_transition_system::<AppState>,
                apply_state_transition_system::<GameState>,
            )
                .in_set(CoreSet::PostUpdate),
        );
    }
}
