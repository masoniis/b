pub mod biome;
pub mod components;
pub mod core;
pub mod terrain;

pub use biome::*;
pub use components::*;
pub use core::{ActiveTerrainGenerator, GeneratedTerrainData, TerrainGenerator};
pub use terrain::*;

// INFO: ----------------------------
//         Terrain gen plugin
// ----------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::generation::core::ActiveBiomeGenerator,
};

pub struct TerrainGenerationPlugin;

impl Plugin for TerrainGenerationPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_resource(ClimateNoiseGenerator::new(0)) // hardcode seed 0 for now
            .add_resource(ActiveBiomeGenerator::default())
            .add_resource(ActiveTerrainGenerator::default());
    }
}
