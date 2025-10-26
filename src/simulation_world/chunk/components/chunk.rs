use crate::prelude::*;
use crate::simulation_world::block::block_registry::BlockId;
use crate::simulation_world::chunk::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH, Y_SHIFT, Z_SHIFT};
use bevy_ecs::prelude::Component;
use std::sync::Arc;

#[derive(Clone, Component)]
pub struct ChunkBlocksComponent {
    blocks: Arc<Vec<BlockId>>,
}

impl ChunkBlocksComponent {
    /// Creates a new empty chunk component filled with air blocks.
    pub fn empty() -> Self {
        Self {
            blocks: Arc::new(vec![0; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH]),
        }
    }

    #[inline(always)]
    /// Gets a reference to the block at the given local coordinates within the chunk.
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&BlockId> {
        if cfg!(debug_assertions) && (x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_DEPTH) {
            error!(
                "get_block: Attempted to access block out of bounds: ({}, {}, {})",
                x, y, z
            );
            return None;
        }

        let index = (y << Y_SHIFT) | (z << Z_SHIFT) | x;

        self.blocks.get(index)
    }

    #[inline(always)]
    /// Sets the block at the given local coordinates within the chunk.
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockId) {
        if cfg!(debug_assertions) && (x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_DEPTH) {
            error!(
                "set_block: Attempted to access block out of bounds: ({}, {}, {})",
                x, y, z
            );
            return;
        }

        let index = (y << Y_SHIFT) | (z << Z_SHIFT) | x;

        // get mutable reference to the blocks vector
        let blocks_mut = Arc::make_mut(&mut self.blocks);
        blocks_mut[index] = block;
    }
}
