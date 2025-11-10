use crate::prelude::*;
use crate::simulation_world::biome::BiomeRegistryResource;
use crate::simulation_world::block::BlockRegistryResource;
use crate::simulation_world::chunk::WorldVoxelPositionIterator;
use crate::simulation_world::terrain::components::{
    climate_map::TerrainClimateMapComponent, BiomeMapComponent,
};
use crate::simulation_world::terrain::generators::core::{PaintResultBuilder, TerrainPainter};

#[derive(Debug, Clone)]
pub struct SimpleSurfacePainter;

impl SimpleSurfacePainter {
    pub fn new() -> Self {
        Self
    }
}

const SEA_LEVEL: i32 = 32;

impl TerrainPainter for SimpleSurfacePainter {
    #[instrument(skip_all)]
    fn paint_terrain_chunk(
        &self,
        mut painter: PaintResultBuilder,
        iterator: WorldVoxelPositionIterator,

        _biome_map: &BiomeMapComponent,
        _climate_map: &TerrainClimateMapComponent,

        block_registry: &BlockRegistryResource,
        _biome_registry: &BiomeRegistryResource,
    ) -> PaintResultBuilder {
        let air_block_id = block_registry.get_block_by_name("air");
        let water_block_id = block_registry.get_block_by_name("water");
        let grass_block_id = block_registry.get_block_by_name("grass");

        let size = painter.get_chunk_size();
        for pos in iterator {
            let (x, y, z) = pos.local;
            let world_y = pos.world.y;

            let current_block_id = painter.get_data_unchecked(x, y, z);

            if current_block_id == air_block_id {
                if world_y < SEA_LEVEL {
                    painter.set_data(x, y, z, water_block_id);
                }
            } else {
                if y < size - 1 {
                    let block_above_id = painter.get_data_unchecked(x, y + 1, z);

                    // if above block is air this is a "surface"
                    if block_above_id == air_block_id {
                        painter.set_data(x, y, z, grass_block_id);
                    }
                } else {
                    // otherwise assume top of chunk just means surface this
                    // is a weak heuristic for cubic chunks though, in future
                    // this should be upgraded to be more thorough
                    painter.set_data(x, y, z, grass_block_id);
                }
            }
        }

        painter
    }
}
