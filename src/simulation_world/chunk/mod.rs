pub mod components;
pub mod generation;
pub mod types;

pub use components::*;
pub use generation::*;
pub use types::*;

// INFO: --------------------------
//         Chunk gen plugin
// --------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{chunk::cube_array::cube_array_generation_system, SimulationSchedule},
};

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems((cube_array_generation_system,));
    }
}
