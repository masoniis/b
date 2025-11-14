use crate::prelude::*;
use crate::simulation_world::{
    chunk::{WorldVoxelPositionIterator, CHUNK_SIDE_LENGTH},
    terrain::{
        components::climate_map::TerrainClimateMapComponent,
        core::ChunkUniformity,
        generators::core::{ShapeResultBuilder, TerrainShaper},
    },
};

/// Generates a simple, rolling terrain using two sine waves.
#[derive(Debug, Clone)]
pub struct SinWaveGenerator {
    /// The average "sea level" height of the terrain.
    base_height: i32,
    /// Controls how high the hills and valleys are.
    amplitude: f32,
    /// Controls how "spread out" the hills are. Smaller values = wider hills.
    frequency: f32,
}

impl SinWaveGenerator {
    pub fn new() -> Self {
        Self {
            base_height: 32, // average world height
            amplitude: 12.0,
            frequency: 0.04,
        }
    }
}

impl TerrainShaper for SinWaveGenerator {
    #[instrument(skip_all)]
    fn determine_chunk_uniformity(&self, coord: IVec3) -> ChunkUniformity {
        let chunk_y_min = coord.y * CHUNK_SIDE_LENGTH as i32;
        let chunk_y_max = (coord.y + 1) * CHUNK_SIDE_LENGTH as i32 - 1;

        let max_variation = self.amplitude * 2.0;
        let max_possible_y = (self.base_height as f32 + max_variation).round() as i32;

        // if above max y, all empty
        if chunk_y_min > max_possible_y {
            return ChunkUniformity::Empty;
        }

        let min_possible_y = (self.base_height as f32 - max_variation).round() as i32;
        let effective_terrain_floor = min_possible_y.max(1);

        // if below sin variation, all solid
        if chunk_y_max < effective_terrain_floor {
            return ChunkUniformity::Solid;
        }

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
        for pos in iterator {
            let (x, y, z) = pos.local;
            let world_x = pos.world.x as f32;
            let world_z = pos.world.z as f32;

            // sin application
            let wave = self.amplitude
                * ((self.frequency * world_x).sin() + (self.frequency * world_z).sin());
            let surface_y = (self.base_height as f32 + wave).round() as i32;

            // block determinance
            let world_y = pos.world.y;
            if world_y < surface_y {
                shaper.mark_as_solid(x, y, z);
            }
        }

        shaper
    }
}
