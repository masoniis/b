use crate::simulation_world::chunk::{CHUNK_SIZE, Y_SHIFT, Z_SHIFT};
use bevy_ecs::component::Component;

/// A representation of the climate data necessary for terrain generation.
#[derive(Debug, Clone, Copy, Default)]
pub struct TerrainClimateData {
    // needed for biome-gen only, thus excluded here!
    // pub temperature: f32,
    // pub precipitation: f32,

    // continuous terrain parameters
    pub continentalness: f32,
    pub erosion: f32,
    pub weirdness: f32,
}

/// Stores the climate data (temperature, precipitation) for every block in a chunk.
#[derive(Component, Clone)]
pub struct ClimateMapComponent(pub [TerrainClimateData; CHUNK_SIZE]);

impl ClimateMapComponent {
    /// Creates a new climate map filled with default values (0.0 for temp/precip).
    pub fn empty() -> Self {
        Self([TerrainClimateData::default(); CHUNK_SIZE])
    }

    /// Gets the climate data for a specific block coordinate within the chunk.
    #[inline(always)]
    pub fn get_climate(&self, x: usize, y: usize, z: usize) -> TerrainClimateData {
        let index = (y << Y_SHIFT) | (z << Z_SHIFT) | x;
        self.0[index]
    }

    /// Sets the climate data for a specific block coordinate within the chunk.
    #[inline(always)]
    pub fn set_climate(&mut self, x: usize, y: usize, z: usize, climate: TerrainClimateData) {
        let index = (y << Y_SHIFT) | (z << Z_SHIFT) | x;
        self.0[index] = climate;
    }
}
