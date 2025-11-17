use crate::prelude::*;
use crate::simulation_world::terrain::climate::ClimateMapComponent;
use crate::simulation_world::{
    chunk::CHUNK_SIDE_LENGTH,
    terrain::generators::shaping::{ChunkUniformity, ShapeResultBuilder, TerrainShaper},
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
        _climate_map: &ClimateMapComponent,

        // output
        mut shaper: ShapeResultBuilder,
    ) -> ShapeResultBuilder {
        let base = self.base_height as f32;
        let amp = self.amplitude;
        let freq = self.frequency;

        shaper.fill_from(|_local, world| {
            let wx = world.x as f32;
            let wz = world.z as f32;

            let wave = amp * ((freq * wx).sin() + (freq * wz).sin());
            let surface_y = (base + wave).round() as i32;

            world.y < surface_y
        });

        shaper
    }
}
