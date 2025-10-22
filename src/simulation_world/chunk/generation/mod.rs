pub mod flat_world;
pub mod superflat_generator;

use crate::simulation_world::chunk::{chunk::Chunk, TransformComponent};

// INFO: ----------------------------------------------
//         Core chunk generation type and trait
// ----------------------------------------------------

use crate::simulation_world::block::property_registry::BlockRegistryResource;
use glam::IVec3;
use std::fmt::Debug;

/// A struct representing generated chunk data.
pub struct GeneratedChunkData {
    pub chunk: Chunk,
    pub transform: TransformComponent,
}

impl GeneratedChunkData {
    pub fn empty() -> Self {
        Self {
            chunk: Chunk::new(),
            transform: TransformComponent::default(),
        }
    }
}

/// A trait for chunk generators to implement.
pub trait ChunkGenerator: Send + Sync + Debug {
    /// Returns generated chunk data for the given chunk coordinates.
    fn generate_chunk(&self, coord: IVec3, blocks: &BlockRegistryResource) -> GeneratedChunkData;
}
