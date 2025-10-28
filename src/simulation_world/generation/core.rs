use crate::simulation_world::biome::BiomeRegistryResource;
use crate::simulation_world::chunk::{ChunkBlocksComponent, ChunkCoord};
use crate::simulation_world::generation::{
    BiomeMapComponent, ClimateNoiseGenerator, DefaultBiomeGenerator, OceanFloorHeightMapComponent,
    TerrainClimateMapComponent, WorldSurfaceHeightMapComponent,
};
use crate::simulation_world::{block::BlockRegistryResource, generation::SuperflatGenerator};
use bevy_ecs::prelude::Resource;
use glam::IVec3;
use std::{fmt::Debug, sync::Arc};

/// A resource holding the active terrain chunk generator.
#[derive(Resource, Clone)]
pub struct ActiveTerrainGenerator(pub Arc<dyn TerrainGenerator>);

impl Default for ActiveTerrainGenerator {
    fn default() -> Self {
        ActiveTerrainGenerator(Arc::new(SuperflatGenerator::new()))
    }
}

/// A resource holding the active biome chunk generator.
#[derive(Resource, Clone)]
pub struct ActiveBiomeGenerator(pub Arc<dyn BiomeGenerator>);

impl Default for ActiveBiomeGenerator {
    fn default() -> Self {
        Self(Arc::new(DefaultBiomeGenerator::default()))
    }
}

// INFO: -------------------------
//         Biome generator
// -------------------------------

/// A trait for just generating the biome map
pub trait BiomeGenerator: Send + Sync + Debug {
    fn generate_biome_chunk(
        &self,
        coord: &ChunkCoord,
        climate_noise: &ClimateNoiseGenerator,
        biome_registry: &BiomeRegistryResource,
    ) -> GeneratedBiomeData;
}

/// A struct representing generated biome data for every block in a chunk.
pub struct GeneratedBiomeData {
    pub biome_map: BiomeMapComponent,
    pub terrain_climate_map: TerrainClimateMapComponent,
}

impl GeneratedBiomeData {
    pub fn empty() -> Self {
        Self {
            biome_map: BiomeMapComponent::empty(),
            terrain_climate_map: TerrainClimateMapComponent::empty(),
        }
    }

    pub fn as_tuple(self) -> (BiomeMapComponent, TerrainClimateMapComponent) {
        (self.biome_map, self.terrain_climate_map)
    }
}

// INFO: ---------------------------
//         Terrain generator
// ---------------------------------

/// A trait for chunk generators to implement.
pub trait TerrainGenerator: Send + Sync + Debug {
    /// Returns generated chunk data for the given chunk coordinates.
    fn generate_terrain_chunk(
        &self,
        coord: IVec3,
        biome_map: &BiomeMapComponent,
        climate_map: &TerrainClimateMapComponent,

        block_registry: &BlockRegistryResource,
        biome_registry: &BiomeRegistryResource,
    ) -> GeneratedTerrainData;
}

/// A struct representing generated chunk data.
pub struct GeneratedTerrainData {
    pub chunk_blocks: ChunkBlocksComponent,
    pub surface_heightmap: OceanFloorHeightMapComponent,
    pub world_surface_heightmap: WorldSurfaceHeightMapComponent,
}

impl GeneratedTerrainData {
    pub fn empty() -> Self {
        Self {
            chunk_blocks: ChunkBlocksComponent::empty(),
            surface_heightmap: OceanFloorHeightMapComponent::empty(),
            world_surface_heightmap: WorldSurfaceHeightMapComponent::empty(),
        }
    }
}

// INFO: -----------------------
//         Bundled types
// -----------------------------

pub struct GeneratedChunkComponentBundle {
    pub chunk_blocks: ChunkBlocksComponent,
    pub biome_map: BiomeMapComponent,
    pub ocean_floor_hmap: OceanFloorHeightMapComponent,
    pub world_surface_hmap: WorldSurfaceHeightMapComponent,
}
