use crate::simulation_world::generation::climate_map::{BiomeClimateData, TerrainClimateData};
use bevy_ecs::resource::Resource;
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex};

#[derive(Resource)]
pub struct ClimateNoiseGenerator {
    // biome-specific noise
    temperature_noise: Fbm<OpenSimplex>,
    precipitation_noise: Fbm<OpenSimplex>,

    // general noise
    continental_noise: Fbm<OpenSimplex>,
    erosion_noise: Fbm<OpenSimplex>,
    weirdness_noise: Fbm<OpenSimplex>,
}

const NOISE_SCALE: f64 = 0.01;

impl ClimateNoiseGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            temperature_noise: create_noise_fn(seed.wrapping_add(3)),
            precipitation_noise: create_noise_fn(seed.wrapping_add(4)),

            continental_noise: create_noise_fn(seed),
            erosion_noise: create_noise_fn(seed.wrapping_add(1)),
            weirdness_noise: create_noise_fn(seed.wrapping_add(2)),
        }
    }

    /// Calculates all 5 climate values for a single world-space block coordinate.
    pub fn get_climate_at(&self, world_pos: glam::IVec3) -> BiomeClimateData {
        let sample_2d = [
            world_pos.x as f64 * NOISE_SCALE,
            world_pos.z as f64 * NOISE_SCALE,
        ];

        // BIOME-ONLY parameters
        let temperature = ((self.temperature_noise.get(sample_2d) + 1.0) * 0.5) as f32;
        let precipitation = ((self.precipitation_noise.get(sample_2d) + 1.0) * 0.5) as f32;

        // BIOME + TERRAIN-GEN parameters
        let continentalness = ((self.continental_noise.get(sample_2d) + 1.0) * 0.5) as f32;
        let erosion = ((self.erosion_noise.get(sample_2d) + 1.0) * 0.5) as f32;
        let weirdness = ((self.weirdness_noise.get(sample_2d) + 1.0) * 0.5) as f32;

        BiomeClimateData {
            temperature,
            precipitation,
            terrain_climate: TerrainClimateData {
                continentalness,
                erosion,
                weirdness,
            },
        }
    }
}

/// Helper function to create a standard FBM noise function
fn create_noise_fn(seed: u32) -> Fbm<OpenSimplex> {
    Fbm::new(seed)
        .set_octaves(6)
        .set_frequency(0.01)
        .set_lacunarity(2.0)
        .set_persistence(0.5)
}
