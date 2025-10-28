use crate::simulation_world::biome::BiomeRegistryResource;
use crate::simulation_world::block::BlockRegistryResource;
use crate::simulation_world::chunk::{
    ChunkBlocksComponent, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH,
};
use crate::simulation_world::generation::{
    BiomeMapComponent, GeneratedTerrainData, OceanFloorHeightMapComponent,
    TerrainClimateMapComponent, TerrainGenerator, WorldSurfaceHeightMapComponent,
};
use glam::IVec3;

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

        let mut chunk = ChunkBlocksComponent::empty();

        for x in 1..CHUNK_WIDTH - 1 {
            for z in 1..CHUNK_DEPTH - 1 {
                for (y, block) in layer_blocks.iter().enumerate() {
                    if y < CHUNK_HEIGHT {
                        chunk.set_block(x, y, z, *block);
                    }
                }

                match biomes.get(biome_map.get_biome(x, 0, z)).name.as_str() {
                    "Ocean" => {
                        chunk.set_block(
                            x,
                            layer_blocks.len(),
                            z,
                            blocks.get_block_by_name("water"),
                        );
                    }
                    _ => {
                        chunk.set_block(
                            x,
                            layer_blocks.len() + 1,
                            z,
                            blocks.get_block_by_name("grass"),
                        );
                    }
                }
            }
        }

        GeneratedTerrainData {
            chunk_blocks: chunk,
            surface_heightmap: OceanFloorHeightMapComponent::empty(),
            world_surface_heightmap: WorldSurfaceHeightMapComponent::empty(),
        }
    }
}
