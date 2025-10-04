use crate::{
    game_world::{schedules::GameSchedule, Plugin, ScheduleBuilder},
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

use super::systems::main as main_system;

pub struct PlayerModulePlugin;

impl Plugin for PlayerModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, _world: &mut World) {
        schedules
            .entry(GameSchedule::Main)
            .add_systems((main_system::camera_control_system,).in_set(CoreSet::Update));
    }
}
