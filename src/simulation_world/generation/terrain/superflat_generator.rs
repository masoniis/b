use crate::prelude::*;
use crate::simulation_world::block::block_registry::BlockId;
use crate::simulation_world::chunk::{WorldVoxelPositionIterator, CHUNK_SIDE_LENGTH};
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    block::BlockRegistryResource,
    chunk::ChunkBlocksComponent,
    generation::{BiomeMapComponent, TerrainClimateMapComponent, TerrainGenerator},
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

    fn sample_block_at(
        &self,
        world_y: i32,
        biome_name: &str,
        layer_blocks: &[BlockId],
        water_block: BlockId,
        dirt_block: BlockId,
        grass_block: BlockId,
        air_block: BlockId,
    ) -> BlockId {
        let num_base_layers = layer_blocks.len();

        if world_y < 0 {
            air_block
        } else if (world_y as usize) < num_base_layers {
            layer_blocks[world_y as usize]
        } else if (world_y as usize) == num_base_layers {
            match biome_name {
                "Ocean" => water_block,
                _ => dirt_block,
            }
        } else if (world_y as usize) == num_base_layers + 1 {
            match biome_name {
                "Ocean" => air_block,
                _ => grass_block,
            }
        } else {
            air_block
        }
    }
}

impl TerrainGenerator for SuperflatGenerator {
    #[instrument(skip_all)]
    fn is_chunk_empty(&self, coord: IVec3) -> bool {
        let chunk_y_min = coord.y * CHUNK_SIDE_LENGTH as i32;
        let chunk_y_max = (coord.y + 1) * CHUNK_SIDE_LENGTH as i32 - 1;

        let world_top_y = (self.base_layers.len() + 2) as i32;
        let world_bottom_y = 0;

        if chunk_y_max < world_bottom_y || chunk_y_min > world_top_y {
            true
        } else {
            false
        }
    }

    #[instrument(skip_all)]
    fn generate_terrain_chunk(
        &self,
        chunk_blocks: &mut ChunkBlocksComponent,
        iterator: WorldVoxelPositionIterator,

        biome_map: &BiomeMapComponent,
        _climate_map: &TerrainClimateMapComponent,
        blocks: &BlockRegistryResource,
        biomes: &BiomeRegistryResource,
    ) {
        let layer_blocks: Vec<_> = self
            .base_layers
            .iter()
            .map(|name| blocks.get_block_by_name(name))
            .collect();

        let water_block = blocks.get_block_by_name("water");
        let grass_block = blocks.get_block_by_name("grass");
        let dirt_block = blocks.get_block_by_name("dirt");
        let air_block = blocks.get_block_by_name("air");

        for pos in iterator {
            let (x, y, z) = pos.local;
            let world_y = pos.world.y;

            let biome_name = &biomes.get(biome_map.get_data_unchecked(x, y, z)).name;
            let block_to_set = self.sample_block_at(
                world_y,
                biome_name.as_str(),
                &layer_blocks,
                water_block,
                dirt_block,
                grass_block,
                air_block,
            );

            chunk_blocks.set_data(x, y, z, block_to_set);
        }
    }
}
