use crate::{
    ecs_modules::{Plugin, Schedules},
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

use super::systems::main as main_system;

pub struct PlayerModuleBuilder;

impl Plugin for PlayerModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        schedules
            .main
            .add_systems((main_system::camera_control_system,).in_set(CoreSet::Update));
    }
}
