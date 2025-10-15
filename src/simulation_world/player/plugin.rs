use crate::{
    ecs_core::{
        state_machine::{utils::in_state, AppState},
        EcsBuilder, Plugin,
    },
    simulation_world::{SimulationSchedule, SimulationSet},
};
use bevy_ecs::prelude::*;

use super::systems::main as main_system;

pub struct PlayerModulePlugin;

impl Plugin for PlayerModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (main_system::camera_control_system.run_if(in_state(AppState::Running)),)
                    .in_set(SimulationSet::Update),
            );
    }
}
