use super::block::Block;
use crate::game_world::world::world_gen::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};

#[derive(Clone)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    blocks: Vec<Block>,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
            blocks: vec![Block { id: 0 }; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH],
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
        if x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_DEPTH {
            return None;
        }
        self.blocks
            .get(y * CHUNK_WIDTH * CHUNK_DEPTH + z * CHUNK_WIDTH + x)
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        if x >= CHUNK_WIDTH || y >= CHUNK_HEIGHT || z >= CHUNK_DEPTH {
            return;
        }
        self.blocks[y * CHUNK_WIDTH * CHUNK_DEPTH + z * CHUNK_WIDTH + x] = block;
    }
}
