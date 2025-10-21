pub mod systems;

// INFO: -----------------------
//         Player plugin
// -----------------------------

use crate::{
    ecs_core::{
        state_machine::{utils::in_state, AppState, GameState},
        EcsBuilder, Plugin,
    },
    simulation_world::{
        player::systems::{camera_control_system, spawn_player_system},
        scheduling::OnEnter,
        SimulationSchedule, SimulationSet,
    },
};
use bevy_ecs::prelude::*;

pub struct PlayerModulePlugin;

impl Plugin for PlayerModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (camera_control_system.run_if(in_state(AppState::Running)),)
                    .in_set(SimulationSet::Update),
            );

        builder
            .schedule_entry(OnEnter(GameState::Playing))
            .add_systems(spawn_player_system);
    }
}
