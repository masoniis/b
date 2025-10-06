use super::systems::*;
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    game_world::schedules::GameSchedule,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(GameSchedule::Startup)
            .add_systems(test_ui_system);

        builder
            .schedule_entry(GameSchedule::Main)
            .add_systems((layout_solver_system,));
    }
}
