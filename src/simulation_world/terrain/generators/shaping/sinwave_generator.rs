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
            base_height: 32, // Average world height
            amplitude: 12.0, // Max height variation from base
            frequency: 0.04, // How "spread out" the waves are
        }
    }
}

impl TerrainShaper for SinWaveGenerator {
    #[instrument(skip_all)]
    fn is_chunk_empty(&self, coord: IVec3) -> bool {
        let chunk_y_min = coord.y * CHUNK_SIDE_LENGTH as i32;
        let chunk_y_max = (coord.y + 1) * CHUNK_SIDE_LENGTH as i32 - 1;

        // max height for sin
        let world_top_y = (self.base_height as f32 + self.amplitude * 2.0).round() as i32;

        // assumed 0 is the bottom of the world here
        let world_bottom_y = 0;

        if chunk_y_max < world_bottom_y || chunk_y_min > world_top_y {
            info!("Culling empty chunk at {coord:?}");
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

        _biome_map: &BiomeMapComponent,
        _climate_map: &TerrainClimateMapComponent,
        _biomes: &BiomeRegistryResource,
    ) -> ShapeResultBuilder {
        for pos in iterator {
            let (x, y, z) = pos.local;
            let world_x = pos.world.x as f32;
            let world_y = pos.world.y;
            let world_z = pos.world.z as f32;

            // sin application
            let wave = self.amplitude
                * ((self.frequency * world_x).sin() + (self.frequency * world_z).sin());
            let surface_y = (self.base_height as f32 + wave).round() as i32;

            // block determinance
            if (world_y > 0) && (world_y < surface_y) {
                shaper.set_filled_blocks(x, y, z);
            }
        }

        shaper
    }
}
