use crate::{
    ecs_core::{EcsBuilder, Plugin},
    game_world::schedules::GameSchedule,
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

use super::systems::main as main_system;

pub struct PlayerModulePlugin;

impl Plugin for PlayerModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(GameSchedule::Main)
            .add_systems((main_system::camera_control_system,).in_set(CoreSet::Update));
    }
}
