use crate::prelude::*;
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    block::BlockRegistryResource,
    chunk::{ChunkBlocksComponent, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH},
    generation::{
        BiomeMapComponent, GeneratedTerrainData, TerrainClimateMapComponent, TerrainGenerator,
    },
};

#[derive(Debug, Clone)]
pub struct SuperflatGenerator {
    base_layers: Vec<String>,
}

impl SuperflatGenerator {
    pub fn new() -> Self {
        Self {
            base_layers: vec!["stone".to_string(), "stone".to_string()],
        }
    }
}

impl TerrainGenerator for SuperflatGenerator {
    #[instrument(skip_all)]
    fn generate_terrain_chunk(
        &self,
        coord: IVec3,
        biome_map: &BiomeMapComponent,
        _climate_map: &TerrainClimateMapComponent,

        blocks: &BlockRegistryResource,
        biomes: &BiomeRegistryResource,
    ) -> GeneratedTerrainData {
        if coord.y != 0 {
            return GeneratedTerrainData::empty();
        }

        let layer_blocks: Vec<_> = self
            .base_layers
            .iter()
            .map(|name| blocks.get_block_by_name(name))
            .collect();

        let water_block = blocks.get_block_by_name("water");
        let grass_block = blocks.get_block_by_name("grass");
        let dirt_block = blocks.get_block_by_name("dirt");

        let mut chunk = ChunkBlocksComponent::empty();

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                for (y, block) in layer_blocks.iter().enumerate() {
                    if y < CHUNK_HEIGHT {
                        chunk.set_block(x, y, z, *block);
                    }
                }

                match biomes.get(biome_map.get_biome(x, 0, z)).name.as_str() {
                    "Ocean" => {
                        chunk.set_block(x, layer_blocks.len(), z, water_block);
                    }
                    _ => {
                        chunk.set_block(x, layer_blocks.len(), z, dirt_block);
                        chunk.set_block(x, layer_blocks.len() + 1, z, grass_block);
                    }
                }
            }
        }

        GeneratedTerrainData {
            chunk_blocks: Some(chunk),
        }
    }
}
