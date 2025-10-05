use super::systems::{main as main_system, startup as startup_system};
use crate::{
    core::world::{EcsBuilder, Plugin},
    game_world::{app_lifecycle::AppState, schedules::GameSchedule, state_machine::in_state},
    prelude::*,
};
use bevy_ecs::prelude::*;

pub struct WorldModulePlugin;

impl Plugin for WorldModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.schedule_entry(GameSchedule::Startup).add_systems((
            // startup_system::chunk_generation_system,
            startup_system::cube_array_generation_system,
        ));

        builder.schedule_entry(GameSchedule::Main).add_systems(
            (main_system::time_system,)
                .in_set(CoreSet::PreUpdate)
                .run_if(in_state(AppState::Running)),
        );
    }
}
