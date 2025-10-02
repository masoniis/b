use super::systems::{self, start_fake_work_system};
use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_modules::state_machine::resources::{AppState, GameState},
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

pub struct StateMachineModuleBuilder;

impl Plugin for StateMachineModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
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
