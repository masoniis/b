use crate::{
    core::{
        state_machine::{systems::apply_state_transition_system, StatePlugin},
        world::{EcsBuilder, Plugin},
    },
    game_world::schedules::GameSchedule,
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

use super::{
    finalize_loading_system, start_fake_work_system, update_app_time_system, AppState,
    AppTimeResource, GameState,
};

pub struct AppLifecyclePlugin;

impl Plugin for AppLifecyclePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(AppTimeResource::default());

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
                update_app_time_system, // TODO: The main system needs to run every frame, even during loading
            )
                .in_set(CoreSet::PostUpdate),
        );
    }
}
