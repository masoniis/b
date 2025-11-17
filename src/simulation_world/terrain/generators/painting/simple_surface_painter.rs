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
        _iterator: WorldVoxelPositionIterator,
        _biome_map: &BiomeMapComponent,
        _climate_map: &TerrainClimateMapComponent,
        block_registry: &BlockRegistryResource,
        _biome_registry: &BiomeRegistryResource,
    ) -> PaintResultBuilder {
        let air_id = block_registry.get_block_by_name("air");
        let water_id = block_registry.get_block_by_name("water");
        let grass_id = block_registry.get_block_by_name("grass");

        let size = painter.size();
        let base_world_pos = painter.chunk_coord.as_world_pos();
        let base_y = base_world_pos.y;
        let chunk_top_y = base_y + size as i32;

        // early exists and optimization
        if let Some(uniform_id) = painter.is_uniform() {
            if uniform_id == air_id {
                if base_y >= SEA_LEVEL {
                    return painter;
                } else if chunk_top_y < SEA_LEVEL {
                    // pure air below sea then fill with water
                    painter.edit_arbitrary(|writer| {
                        writer.fill(water_id);
                    });
                    return painter;
                }
            }
            // skip full solids
            // TODO: technically a bug if the chunk is uniform but still located at
            // the surface. This is a bit rare and i'm currently lazy to address it
            if uniform_id != air_id {
                return painter;
            }
        }

        // chunk is mixed
        painter.edit_arbitrary(|writer| {
            for x in 0..size {
                for z in 0..size {
                    for y in 0..size {
                        let world_y = base_y + y as i32;
                        let current_block_id = writer.get_block(x, y, z);

                        // water fill
                        if current_block_id == air_id {
                            if world_y < SEA_LEVEL {
                                writer.set_block(x, y, z, water_id);
                            }
                            continue;
                        }

                        // grass paint (top block never gets checked though, part of the todo above)
                        if y < size - 1 {
                            let block_above_id = writer.get_block(x, y + 1, z);

                            if block_above_id == air_id {
                                if world_y >= SEA_LEVEL {
                                    writer.set_block(x, y, z, grass_id);
                                }
                            }
                        }
                    }
                }
            }
        });

        painter
    }
}
