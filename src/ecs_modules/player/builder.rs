use crate::ecs_bridge::{Plugin, Schedules};
use bevy_ecs::world::World;

use super::systems::main as main_system;

pub struct PlayerModuleBuilder;

impl Plugin for PlayerModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        schedules
            .main
            .add_systems((main_system::camera_control_system,));
    }
}
