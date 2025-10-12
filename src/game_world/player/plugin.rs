use crate::{
    ecs_core::{in_state, EcsBuilder, Plugin},
    game_world::{app_lifecycle::AppState, schedules::GameSchedule},
    prelude::GameSet,
};
use bevy_ecs::prelude::*;

use super::systems::main as main_system;

pub struct PlayerModulePlugin;

impl Plugin for PlayerModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.schedule_entry(GameSchedule::Main).add_systems(
            (main_system::camera_control_system.run_if(in_state(AppState::Running)),)
                .in_set(GameSet::Update),
        );
    }
}
