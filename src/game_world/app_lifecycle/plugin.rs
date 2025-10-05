use crate::{
    core::world::{EcsBuilder, Plugin},
    game_world::{
        schedules::GameSchedule,
        state_machine::{systems::apply_state_transition_system, StatePlugin},
    },
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

use super::{finalize_loading_system, start_fake_work_system, AppState, GameState};

pub struct AppLifecyclePlugin;

impl Plugin for AppLifecyclePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_plugin(StatePlugin::<AppState>::default())
            .add_plugin(StatePlugin::<GameState>::default());

        // The state resources the state machine oversees
        builder
            .schedule_entry(GameSchedule::Startup)
            .add_systems(start_fake_work_system);

        builder.schedule_entry(GameSchedule::Loading).add_systems(
            (
                finalize_loading_system,
                apply_state_transition_system::<AppState>,
                crate::game_world::world::systems::main::time::time_system, // Add time_system
            )
                .chain(),
        );

        builder.schedule_entry(GameSchedule::Main).add_systems(
            (
                apply_state_transition_system::<AppState>,
                apply_state_transition_system::<GameState>,
            )
                .in_set(CoreSet::PostUpdate),
        );
    }
}
