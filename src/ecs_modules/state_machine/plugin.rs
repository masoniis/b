use super::systems::{self, start_fake_work_system};
use crate::{
    ecs_modules::{
        graphics,
        state_machine::resources::{AppState, GameState},
        Plugin, Schedules,
    },
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

pub struct StateMachineModulePlugin;

impl Plugin for StateMachineModulePlugin {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        // Add systems to the regular schedules
        schedules.startup.add_systems(start_fake_work_system);

        schedules.loading.add_systems(
            (
                systems::finalize_loading_system,
                systems::apply_state_transition_system::<AppState>,
                graphics::systems::render::render_loading_screen_system,
                crate::ecs_modules::world::systems::main::time::time_system, // Add time_system
            )
                .chain(),
        );

        schedules.main.add_systems(
            (
                systems::apply_state_transition_system::<AppState>,
                systems::apply_state_transition_system::<GameState>,
            )
                .in_set(CoreSet::PostUpdate),
        );
    }
}
