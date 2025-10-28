use crate::prelude::*;
use glam::IVec3;
use std::fmt::Debug;

use crate::simulation_world::{
    biome::BiomeRegistryResource,
    chunk::{ChunkCoord, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH},
    generation::{
        core::{BiomeGenerator, GeneratedBiomeData},
        BiomeMapComponent, ClimateNoiseGenerator, TerrainClimateMapComponent,
    },
};

// A default implementation
#[derive(Debug, Default)]
pub struct DefaultBiomeGenerator;

impl BiomeGenerator for DefaultBiomeGenerator {
    #[instrument(skip_all)]
    fn generate_biome_chunk(
        &self,
        coord: &ChunkCoord,
        climate_noise: &ClimateNoiseGenerator,
        biome_registry: &BiomeRegistryResource,
    ) -> GeneratedBiomeData {
        // maps to populate
        let mut biome_map = BiomeMapComponent::empty();
        let mut terrain_climate_map = TerrainClimateMapComponent::empty();

        let plains_id = biome_registry.get_biome_id_or_default("plains");
        let ocean_id = biome_registry.get_biome_id_or_default("ocean");

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                // generate 2d noise params
                let world_pos = coord.get_block_world_pos(IVec3::new(x as i32, 0 as i32, z as i32));

                // FIXME: this line is causing the slow down
                let climate_data = climate_noise.get_climate_at(world_pos.x, world_pos.z);
                terrain_climate_map.set_climate(x, z, climate_data.terrain_climate);
                let (temperature, _precipitation) =
                    (climate_data.temperature, climate_data.precipitation);

                for y in 0..CHUNK_HEIGHT {
                    // INFO: --------------------------
                    //         Determine biomes
                    // --------------------------------

                    if temperature >= 0.5 {
                        biome_map.set_biome(x, y, z, plains_id);
                    } else {
                        biome_map.set_biome(x, y, z, ocean_id);
                    }
                }
            }
        }

        return GeneratedBiomeData {
            biome_map,
            terrain_climate_map,
        };
    }
}
