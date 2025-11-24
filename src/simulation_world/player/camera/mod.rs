pub mod component;
pub mod movement;
pub mod resource;

pub use component::*;
pub use movement::*;
pub use resource::*;

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
        builder.init_resource::<ActiveCamera>();

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
