use super::systems::{self, start_fake_work_system, transition_state::OnExit};
use crate::{
    ecs_modules::{
        schedules::OnEnter,
        state_machine::resources::{AppState, GameState},
        Plugin, Schedules,
    },
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

pub struct StateMachineModuleBuilder;

impl Plugin for StateMachineModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        // Initialize the transition schedules
        schedules.add(OnEnter(AppState::Running));
        schedules.add(OnExit(AppState::Running));
        schedules.add(OnEnter(AppState::Loading));
        schedules.add(OnExit(AppState::Loading));
        schedules.add(OnEnter(AppState::Closing));
        schedules.add(OnExit(AppState::Closing));

        // Add systems to the regular schedules
        schedules.startup.add_systems(start_fake_work_system);

        schedules.loading.add_systems((
            systems::finalize_loading_system,
            systems::apply_state_transition_system::<AppState>
                .after(systems::finalize_loading_system),
        ));

        schedules.main.add_systems(
            (
                systems::apply_state_transition_system::<AppState>,
                systems::apply_state_transition_system::<GameState>,
            )
                .in_set(CoreSet::PostUpdate),
        );
    }
}
