use super::systems::{main as main_system, startup as startup_system};
use crate::{
    game_world::state_machine::in_state,
    game_world::state_machine::resources::AppState,
    game_world::{schedules::GameSchedule, Plugin, ScheduleBuilder},
    prelude::*,
};
use bevy_ecs::prelude::*;

pub struct WorldModulePlugin;

impl Plugin for WorldModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, _world: &mut World) {
        schedules.entry(GameSchedule::Startup).add_systems((
            // startup_system::chunk_generation_system,
            startup_system::cube_array_generation_system,
        ));

        schedules.entry(GameSchedule::Main).add_systems(
            (main_system::time_system,)
                .in_set(CoreSet::PreUpdate)
                .run_if(in_state(AppState::Running)),
        );
    }
}
