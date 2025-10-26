use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::{BiomeMap, ChunkComponent, SuperflatGenerator};
use bevy_ecs::prelude::Resource;
use glam::IVec3;
use std::{fmt::Debug, sync::Arc};

/// A resource holding the active chunk generator.
#[derive(Resource, Clone)]
pub struct ActiveChunkGenerator(pub Arc<dyn ChunkGenerator>);

impl Default for ActiveChunkGenerator {
    fn default() -> Self {
        ActiveChunkGenerator(Arc::new(SuperflatGenerator::new()))
    }
}

/// A trait for chunk generators to implement.
pub trait ChunkGenerator: Send + Sync + Debug {
    /// Returns generated chunk data for the given chunk coordinates.
    fn generate_chunk(&self, coord: IVec3, blocks: &BlockRegistryResource) -> GeneratedChunkData;
}

/// A struct representing generated chunk data.
pub struct GeneratedChunkData {
    pub chunk_component: ChunkComponent,
    pub biome_map: BiomeMap,
}

impl GeneratedChunkData {
    pub fn empty() -> Self {
        Self {
            chunk_component: ChunkComponent::empty(),
            biome_map: BiomeMap::empty(),
        }
    }
}
