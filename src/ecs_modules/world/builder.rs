use super::systems::{main as main_system, startup as startup_system};
use crate::ecs_bridge::{Plugin, Schedules};
use crate::prelude::CoreSet;
use bevy_ecs::prelude::*;

pub struct WorldModuleBuilder;

impl Plugin for WorldModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        schedules.startup.add_systems((
            startup_system::chunk_generation_system,
            startup_system::cube_array_generation_system,
        ));

        schedules
            .main
            .add_systems((main_system::time_system,).in_set(CoreSet::PreUpdate));
    }
}
