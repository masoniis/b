use crate::ecs_bridge::{Plugin, Schedules};
use bevy_ecs::world::World;

use super::main_system;

pub struct RenderingModuleBuilder;

impl Plugin for RenderingModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        schedules.main.add_systems((
            main_system::changed_mesh_system,
            main_system::removed_mesh_system,
        ));
    }
}
