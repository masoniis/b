use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::{
    BiomeMap, ChunkComponent, ChunkGenerator, GeneratedChunkData, CHUNK_DEPTH, CHUNK_HEIGHT,
    CHUNK_WIDTH,
};
use glam::IVec3;

#[derive(Debug, Clone)]
pub struct SuperflatGenerator {
    layers: Vec<String>,
}

impl SuperflatGenerator {
    pub fn new() -> Self {
        Self {
            layers: vec![
                "stone".to_string(),
                "lava".to_string(),
                "stone".to_string(),
                "grass".to_string(),
            ],
        }
    }
}

impl ChunkGenerator for SuperflatGenerator {
    fn generate_chunk(&self, coord: IVec3, blocks: &BlockRegistryResource) -> GeneratedChunkData {
        if coord.y != 0 {
            return GeneratedChunkData::empty();
        }

        let layer_blocks: Vec<_> = self
            .layers
            .iter()
            .map(|name| blocks.get_block_by_name(name))
            .collect();

        let mut chunk = ChunkComponent::empty();

        for x in 1..CHUNK_WIDTH - 1 {
            for z in 1..CHUNK_DEPTH - 1 {
                for (y, block) in layer_blocks.iter().enumerate() {
                    if y < CHUNK_HEIGHT {
                        chunk.set_block(x, y, z, *block);
                    }
                }
            }
        }

        GeneratedChunkData {
            chunk_component: chunk,
            biome_map: BiomeMap::empty(),
        }
    }
}
