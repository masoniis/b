use super::block::Block;
use crate::prelude::*;
use crate::simulation_world::chunk::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH, Y_SHIFT, Z_SHIFT};
use bevy_ecs::prelude::Component;

#[derive(Clone, Component)]
pub struct ChunkComponent {
    blocks: Vec<Block>,
}

impl ChunkComponent {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block { id: 0 }; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH],
        }
    }

    #[inline(always)]
    /// Gets a reference to the block at the given local coordinates within the chunk.
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
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
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        if cfg!(debug_assertions) && (x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_DEPTH) {
            error!(
                "set_block: Attempted to access block out of bounds: ({}, {}, {})",
                x, y, z
            );
            return;
        }

        let index = (y << Y_SHIFT) | (z << Z_SHIFT) | x;

        self.blocks[index] = block;
    }
}
