use crate::prelude::*;
use crate::simulation_world::chunk::{WorldVoxelPositionIterator, CHUNK_SIDE_LENGTH};
use crate::simulation_world::terrain::generators::core::ShapeResultBuilder;
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    terrain::{
        components::{climate_map::TerrainClimateMapComponent, BiomeMapComponent},
        generators::core::TerrainShaper,
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
    fn is_chunk_empty(&self, coord: IVec3) -> bool {
        let chunk_y_min = coord.y * CHUNK_SIDE_LENGTH as i32;

        let world_top_y = self.land_height;
        if chunk_y_min > world_top_y {
            true
        } else {
            false
        }
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
                shaper.set_filled_blocks(x, y, z);
            }
        }

        shaper
    }
}
