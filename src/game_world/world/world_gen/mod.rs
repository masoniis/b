use crate::game_world::world::utils::types::{block::Block, chunk::Chunk};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_DEPTH: usize = 16;

const STONE: Block = Block { id: 1 };
const GRASS: Block = Block { id: 2 };

pub fn generate_flat_world_chunk(chunk: &mut Chunk) {
    for x in 0..CHUNK_WIDTH {
        for z in 0..CHUNK_DEPTH {
            for y in 0..CHUNK_HEIGHT {
                if y < CHUNK_HEIGHT - 1 {
                    chunk.set_block(x, y, z, STONE);
                } else {
                    chunk.set_block(x, y, z, GRASS);
                }
            }
        }
    }
}
