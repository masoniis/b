use std::fmt::Debug;

use glam::IVec3;

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
    fn generate_biome_chunk(
        &self,
        coord: &ChunkCoord,
        biome_registry: &BiomeRegistryResource,
    ) -> GeneratedBiomeData {
        // seeded generator
        let climate_gen = ClimateNoiseGenerator::new(0);

        // maps to populate
        let mut biome_map = BiomeMapComponent::empty();
        let mut terrain_climate_map = TerrainClimateMapComponent::empty();

        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                for x in 0..CHUNK_WIDTH {
                    let world_pos =
                        coord.get_block_world_pos(IVec3::new(x as i32, y as i32, z as i32));

                    let climate_data = climate_gen.get_climate_at(world_pos);
                    terrain_climate_map.set_climate(x, y, z, climate_data.terrain_climate);

                    let (temperature, _precipitation) =
                        (climate_data.temperature, climate_data.precipitation);

                    // INFO: --------------------------
                    //         Determine biomes
                    // --------------------------------

                    if temperature >= 0.5 {
                        biome_map.set_biome(
                            x,
                            y,
                            z,
                            biome_registry.get_biome_id_or_default("plains"),
                        );
                    } else {
                        biome_map.set_biome(
                            x,
                            y,
                            z,
                            biome_registry.get_biome_id_or_default("ocean"),
                        );
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
