pub mod camera;
pub mod camera_movement;
pub mod default_camera;

pub use camera::*;
pub use camera_movement::*;
pub use default_camera::*;

// INFO: -----------------------
//         Camera plugin
// -----------------------------

use crate::{
    ecs_core::{
        state_machine::{utils::in_state, AppState},
        EcsBuilder, Plugin,
    },
    simulation_world::{SimulationSchedule, SimulationSet},
};
use bevy_ecs::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(setup_camera_system);

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (camera_movement_system, update_camera_chunk_chord_system)
                    .chain()
                    .run_if(in_state(AppState::Running))
                    .in_set(SimulationSet::Update),
            );
    }
}
