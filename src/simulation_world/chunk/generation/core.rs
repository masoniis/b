use crate::simulation_world::block::BlockRegistryResource;
use crate::simulation_world::chunk::height_maps::{SurfaceHeightmap, WorldSurfaceHeightmap};
use crate::simulation_world::chunk::{
    BiomeMap, ChunkBlocksComponent, ClimateMap, DefaultBiomeGenerator, SuperflatGenerator,
};
use bevy_ecs::prelude::Resource;
use glam::IVec3;
use std::{fmt::Debug, sync::Arc};

/// A resource holding the active chunk generator.
#[derive(Resource, Clone)]
pub struct ActiveChunkGenerator(pub Arc<dyn TerrainGenerator>);

impl Default for ActiveChunkGenerator {
    fn default() -> Self {
        ActiveChunkGenerator(Arc::new(SuperflatGenerator::new()))
    }
}

/// A resource holding the active biome generator.
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
    fn generate_biome_data(&self, coord: IVec3) -> GeneratedBiomeData;
}

/// A struct representing generated biome data for every block in a chunk.
pub struct GeneratedBiomeData {
    pub biome_map: BiomeMap,
    pub climate_map: ClimateMap,
}

impl GeneratedBiomeData {
    pub fn empty() -> Self {
        Self {
            biome_map: BiomeMap::empty(),
            climate_map: ClimateMap::empty(),
        }
    }

    pub fn as_tuple(self) -> (BiomeMap, ClimateMap) {
        (self.biome_map, self.climate_map)
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
        biome_map: &BiomeMap,
        climate_map: &ClimateMap,
        block_registry: &BlockRegistryResource,
    ) -> GeneratedTerrainData;
}

/// A struct representing generated chunk data.
pub struct GeneratedTerrainData {
    pub chunk_blocks: ChunkBlocksComponent,
    pub surface_heightmap: SurfaceHeightmap,
    pub world_surface_heightmap: WorldSurfaceHeightmap,
}

impl GeneratedTerrainData {
    pub fn empty() -> Self {
        Self {
            chunk_blocks: ChunkBlocksComponent::empty(),
            surface_heightmap: SurfaceHeightmap::empty(),
            world_surface_heightmap: WorldSurfaceHeightmap::empty(),
        }
    }
}

// INFO: -----------------------
//         Bundled types
// -----------------------------

pub struct GeneratedChunkComponentBundle {
    pub chunk_blocks: ChunkBlocksComponent,
    pub biome_map: BiomeMap,
    pub surface_heightmap: SurfaceHeightmap,
    pub world_surface_heightmap: WorldSurfaceHeightmap,
}
