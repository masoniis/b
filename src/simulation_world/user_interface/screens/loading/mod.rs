pub mod loading_screen;

pub use loading_screen::{despawn_loading_ui_system, spawn_loading_ui_system};

// INFO: ----------------
//         Plugin
// ----------------------

use crate::{
    ecs_core::{state_machine::AppState, EcsBuilder, Plugin},
    simulation_world::{scheduling::StartupSet, OnExit, SimulationSchedule},
};
use bevy_ecs::prelude::*;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(spawn_loading_ui_system.in_set(StartupSet::Tasks));

        builder
            .schedule_entry(OnExit(AppState::StartingUp))
            .add_systems(despawn_loading_ui_system);
    }
}
