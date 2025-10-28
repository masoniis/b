use crate::simulation_world::chunk::{CHUNK_AREA, Z_SHIFT};
use bevy_ecs::component::Component;

// INFO: --------------------------------
//         Biome gen climate data
// --------------------------------------

/// A representation of the climate data necessary for biome generation.
///
/// Includes all the parameters in terrain climate data, plus temperature and precipitation.
#[derive(Debug, Clone, Copy, Default)]
pub struct BiomeClimateData {
    pub temperature: f32,
    pub precipitation: f32,

    pub terrain_climate: TerrainClimateData,
}

// INFO: ----------------------------------
//         Terrain gen climate data
// ----------------------------------------

/// A representation of the climate data necessary for terrain generation.
///
/// Includes continuous values for continentalness, erosion, and weirdness.
#[derive(Debug, Clone, Copy, Default)]
pub struct TerrainClimateData {
    pub continentalness: f32,
    pub erosion: f32,
    pub weirdness: f32,
}

/// Stores the climate data (temperature, precipitation) for every COLUMN in a chunk.
#[derive(Component, Clone)]
pub struct TerrainClimateMapComponent(pub [TerrainClimateData; CHUNK_AREA]);

impl TerrainClimateMapComponent {
    /// Creates a new climate map filled with default values (0.0 for temp/precip).
    pub fn empty() -> Self {
        Self([TerrainClimateData::default(); CHUNK_AREA])
    }

    /// Gets the climate data for a specific block coordinate within the chunk.
    #[inline(always)]
    pub fn get_climate(&self, x: usize, z: usize) -> TerrainClimateData {
        let index = (z << Z_SHIFT) | x;
        self.0[index]
    }

    /// Sets the climate data for a specific block coordinate within the chunk.
    #[inline(always)]
    pub fn set_climate(&mut self, x: usize, z: usize, climate: TerrainClimateData) {
        let index = (z << Z_SHIFT) | x;
        self.0[index] = climate;
    }
}
