use crate::prelude::*;
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    chunk::{WorldVoxelPositionIterator, CHUNK_SIDE_LENGTH},
    terrain::{
        components::{climate_map::TerrainClimateMapComponent, BiomeMapComponent},
        core::ChunkUniformity,
        generators::core::{ShapeResultBuilder, TerrainShaper},
    },
};

#[derive(Debug, Clone)]
pub struct HeightmapShaper {
    land_height: i32,
    ocean_height: i32,
}

impl HeightmapShaper {
    pub fn new() -> Self {
        Self {
            land_height: 32,
            ocean_height: 25,
        }
    }
}

impl TerrainShaper for HeightmapShaper {
    #[instrument(skip_all)]
    fn determine_chunk_uniformity(&self, coord: IVec3) -> ChunkUniformity {
        let chunk_y_min = coord.y * CHUNK_SIDE_LENGTH as i32;
        let chunk_y_max = (coord.y + 1) * CHUNK_SIDE_LENGTH as i32 - 1;

        let world_top_y = self.land_height;
        let world_bottom_y = self.ocean_height;

        // if above highest, empty
        if chunk_y_min > world_top_y {
            return ChunkUniformity::Empty;
        }

        // if below lowest, solid
        if chunk_y_max < world_bottom_y {
            return ChunkUniformity::Solid;
        }

        // otherwise mixed
        ChunkUniformity::Mixed
    }

    #[instrument(skip_all)]
    fn shape_terrain_chunk(
        &self,
        mut shaper: ShapeResultBuilder,
        iterator: WorldVoxelPositionIterator,

        biome_map: &BiomeMapComponent,
        _climate_map: &TerrainClimateMapComponent,
        biomes: &BiomeRegistryResource,
    ) -> ShapeResultBuilder {
        let mut surface_height = self.land_height;

        for pos in iterator {
            let (x, y, z) = pos.local;
            let world_y = pos.world.y;

            // if new column started
            if y == 0 {
                let biome_name = &biomes.get(biome_map.get_data_unchecked(x, 0, z)).name;

                // update height based on biome
                surface_height = if biome_name.as_str() == "Ocean" {
                    self.ocean_height
                } else {
                    self.land_height
                };
            }

            if world_y <= surface_height {
                shaper.mark_as_solid(x, y, z);
            }
        }

        shaper
    }
}
