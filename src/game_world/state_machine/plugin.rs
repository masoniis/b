use super::{
    resources::{CurrentState, NextState, PrevState},
    systems::{self, start_fake_work_system},
};
use crate::{
    game_world::{
        graphics,
        schedules::GameSchedule,
        state_machine::resources::{AppState, GameState},
        Plugin, ScheduleBuilder,
    },
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

pub struct StateMachineModulePlugin;

impl Plugin for StateMachineModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, world: &mut World) {
        // The state resources the state machine oversees
        world.insert_resource(PrevState {
            val: None::<AppState>,
        });
        world.insert_resource(PrevState {
            val: None::<GameState>,
        });
        world.insert_resource(CurrentState {
            val: AppState::default(),
        });
        world.insert_resource(CurrentState {
            val: GameState::default(),
        });
        world.insert_resource(NextState {
            val: None::<AppState>,
        });
        world.insert_resource(NextState {
            val: None::<GameState>,
        });

        // Add systems to the regular schedules
        schedules
            .entry(GameSchedule::Startup)
            .add_systems(start_fake_work_system);

        schedules.entry(GameSchedule::Loading).add_systems(
            (
                systems::finalize_loading_system,
                systems::apply_state_transition_system::<AppState>,
                crate::game_world::world::systems::main::time::time_system, // Add time_system
            )
                .chain(),
        );

        schedules.entry(GameSchedule::Main).add_systems(
            (
                systems::apply_state_transition_system::<AppState>,
                systems::apply_state_transition_system::<GameState>,
            )
                .in_set(CoreSet::PostUpdate),
        );
    }
}
