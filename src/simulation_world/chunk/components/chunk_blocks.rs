use crate::prelude::*;
use crate::simulation_world::block::block_registry::{BlockId, AIR_BLOCK_ID};
use crate::simulation_world::chunk::types::{ChunkLod, ChunkVolumeData};
use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Deref, DerefMut)]
pub struct ChunkBlocksComponent(pub ChunkVolumeData<BlockId>);

impl ChunkBlocksComponent {
    pub fn new(lod: ChunkLod, block_data: Vec<BlockId>) -> Self {
        Self(ChunkVolumeData::new(lod, block_data))
    }

    pub fn new_empty(lod: ChunkLod) -> Self {
        Self(ChunkVolumeData::new_filled(lod, AIR_BLOCK_ID))
    }

    pub fn new_filled(lod: ChunkLod, block_id: BlockId) -> Self {
        Self(ChunkVolumeData::new_filled(lod, block_id))
    }
}
