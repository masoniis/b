use crate::prelude::*;
use crate::simulation_world::{
    chunk::{WorldVoxelPositionIterator, CHUNK_SIDE_LENGTH},
    terrain::{
        components::climate_map::TerrainClimateMapComponent, // This is no longer used, but kept to match the trait
        core::ChunkUniformity,
        generators::core::{ShapeResultBuilder, TerrainShaper},
    },
};

#[derive(Debug, Clone)]
pub struct SuperflatShaper {
    land_height: i32,
}

impl SuperflatShaper {
    pub fn new() -> Self {
        Self { land_height: 32 }
    }
}

impl TerrainShaper for SuperflatShaper {
    #[instrument(skip_all)]
    fn determine_chunk_uniformity(&self, coord: IVec3) -> ChunkUniformity {
        let chunk_y_min = coord.y * CHUNK_SIDE_LENGTH as i32;
        let chunk_y_max = (coord.y + 1) * CHUNK_SIDE_LENGTH as i32 - 1;
        let world_surface_y = self.land_height;

        // if above highest, empty
        if chunk_y_min > world_surface_y {
            return ChunkUniformity::Empty;
        }

        // if below lowest, solid
        if chunk_y_max < world_surface_y {
            return ChunkUniformity::Solid;
        }

        // otherwise mixed (only the chunk layer at the surface height)
        ChunkUniformity::Mixed
    }

    #[instrument(skip_all)]
    fn shape_terrain_chunk(
        &self,
        // input
        iterator: WorldVoxelPositionIterator,
        _climate_map: &TerrainClimateMapComponent,

        // output
        mut shaper: ShapeResultBuilder,
    ) -> ShapeResultBuilder {
        let surface_height = self.land_height;

        for pos in iterator {
            let (x, y, z) = pos.local;
            let world_y = pos.world.y;

            if world_y <= surface_height {
                shaper.mark_as_solid(x, y, z);
            }
        }

        shaper
    }
}
