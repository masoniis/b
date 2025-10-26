use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::biome_map::BiomeId;
use crate::simulation_world::chunk::height_maps::{SurfaceHeightmap, WorldSurfaceHeightmap};
use crate::simulation_world::chunk::{
    BiomeMap, ChunkBlocksComponent, SuperflatGenerator, CHUNK_AREA,
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

// A trait for just generating the biome map
pub trait BiomeGenerator: Send + Sync + Debug {
    fn generate_biome_map(&self, coord: IVec3) -> BiomeMap;
}

// A default implementation
#[derive(Debug, Default)]
pub struct DefaultBiomeGenerator;

impl BiomeGenerator for DefaultBiomeGenerator {
    fn generate_biome_map(&self, _coord: IVec3) -> BiomeMap {
        BiomeMap([BiomeId::Plains; CHUNK_AREA])
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
        blocks: &BlockRegistryResource,
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
