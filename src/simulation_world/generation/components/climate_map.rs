use crate::prelude::*;
use crate::simulation_world::chunk::{types::ChunkLod, ChunkColumnData};
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
#[derive(Component, Clone, Deref, DerefMut)]
pub struct TerrainClimateMapComponent(pub ChunkColumnData<TerrainClimateData>);

impl TerrainClimateMapComponent {
    /// Creates a new climate map filled with default values (0.0 for temp/precip).
    pub fn new_empty(lod: ChunkLod) -> Self {
        Self(ChunkColumnData::new_filled(
            lod,
            TerrainClimateData::default(),
        ))
    }
}
