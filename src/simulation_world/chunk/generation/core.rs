use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::{chunk::ChunkComponent, TransformComponent};
use bevy_ecs::prelude::Resource;
use glam::IVec3;
use std::fmt::Debug;
use std::sync::Arc;

/// A resource holding the active chunk generator.
#[derive(Resource, Clone)]
pub struct ActiveChunkGenerator(pub Arc<dyn ChunkGenerator>);

/// A trait for chunk generators to implement.
pub trait ChunkGenerator: Send + Sync + Debug {
    /// Returns generated chunk data for the given chunk coordinates.
    fn generate_chunk(&self, coord: IVec3, blocks: &BlockRegistryResource) -> GeneratedChunkData;
}

/// A struct representing generated chunk data.
pub struct GeneratedChunkData {
    pub chunk_component: ChunkComponent,
    pub transform_component: TransformComponent,
}

impl GeneratedChunkData {
    pub fn empty() -> Self {
        Self {
            chunk_component: ChunkComponent::new(),
            transform_component: TransformComponent::default(),
        }
    }
}
