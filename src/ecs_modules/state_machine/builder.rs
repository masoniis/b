use super::systems;
use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_modules::state_machine::resources::{AppState, GameState},
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

pub struct StateMachineModuleBuilder;

impl Plugin for StateMachineModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        schedules.main.add_systems(
            (
                systems::apply_state_transition_system::<AppState>,
                systems::apply_state_transition_system::<GameState>,
            )
                .in_set(CoreSet::PostUpdate),
        );
    }
}
