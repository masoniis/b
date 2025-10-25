use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::SuperflatGenerator;
use crate::simulation_world::chunk::{chunk::ChunkComponent, TransformComponent};
use bevy_ecs::prelude::Resource;
use glam::IVec3;
use std::fmt::Debug;
use std::sync::Arc;

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
    fn generate_chunk(
        &self,
        coord: IVec3,
        blocks: &BlockRegistryResource,
    ) -> GeneratedChunkComponents;
}

/// A struct representing generated chunk data.
pub struct GeneratedChunkComponents {
    pub chunk_component: ChunkComponent,
    pub transform_component: TransformComponent,
}

impl GeneratedChunkComponents {
    pub fn empty(coord: IVec3) -> Self {
        Self {
            chunk_component: ChunkComponent::new(coord),
            transform_component: TransformComponent::default(),
        }
    }
}
