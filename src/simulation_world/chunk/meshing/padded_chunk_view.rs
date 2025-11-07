use crate::prelude::*;
use crate::simulation_world::{
    block::block_registry::{BlockId, AIR_BLOCK_ID},
    chunk::{ChunkBlocksComponent, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH},
};
use glam::IVec3;

/// Represents the state of neighbor chunk data passed to the mesher.
#[derive(Clone, Default)]
pub enum ChunkDataOption {
    /// The chunk's block data is available.
    Generated(ChunkBlocksComponent),
    /// The chunk coordinate is outside the world's bounds.
    OutOfBounds,
    /// The chunk is within bounds but has no data (e.g., not generated).
    #[default]
    Empty,
}

/// A 3x3x3 view of chunk data, centered on the chunk being meshed.
#[derive(Clone)]
pub struct PaddedChunkView {
    /// A 3x3x3 grid of chunk block data.
    /// Index [1][1][1] is the center chunk being meshed.
    /// Index [0][1][1] is the -X (left) neighbor.
    /// Index [2][1][1] is the +X (right) neighbor.
    chunks: [[[ChunkDataOption; 3]; 3]; 3],
}

impl PaddedChunkView {
    /// Creates a new view from a 3x3x3 array of chunk data.
    pub fn new(chunks: [[[ChunkDataOption; 3]; 3]; 3]) -> Self {
        Self { chunks }
    }

    /// Gets a block ID from the padded view. A value higher or lower than
    /// the chunk dimensions will access neighboring chunks.
    ///
    /// WARN: Assumes valid input for runtime efficiency, but if you exit
    /// the neighbor chunk undefined behavior may occur.
    pub fn get_block(&self, pos: IVec3) -> BlockId {
        const W: i32 = CHUNK_WIDTH as i32;
        const H: i32 = CHUNK_HEIGHT as i32;
        const D: i32 = CHUNK_DEPTH as i32;

        if cfg!(debug_assertions)
            && (pos.x < -W
                || pos.x >= 2 * W
                || pos.y < -H
                || pos.y >= 2 * H
                || pos.z < -D
                || pos.z >= 2 * D)
        {
            error!(
                "get_block: Attempted to access block out of padded chunk bounds: ({}, {}, {})",
                pos.x, pos.y, pos.z
            );
        }

        // determine chunk to read from (0, 1, or 2)
        let chunk_idx_x = (pos.x.div_euclid(W) + 1) as usize;
        let chunk_idx_y = (pos.y.div_euclid(H) + 1) as usize;
        let chunk_idx_z = (pos.z.div_euclid(D) + 1) as usize;

        // get the chunk block
        match &self.chunks[chunk_idx_x][chunk_idx_y][chunk_idx_z] {
            ChunkDataOption::Generated(chunk) => {
                // modular arithmetic remainder to "wrap" for other chunks
                let local_x = pos.x.rem_euclid(W) as usize;
                let local_y = pos.y.rem_euclid(H) as usize;
                let local_z = pos.z.rem_euclid(D) as usize;

                // get block from the chunk
                *chunk.get_unchecked_block(local_x, local_y, local_z)
            }
            ChunkDataOption::OutOfBounds => {
                // a chunk outside the world, return solid block
                // id so that faces are never drawn against it
                1
            }
            ChunkDataOption::Empty => {
                // a chunk inside the world, but empty, return air
                AIR_BLOCK_ID
            }
        }
    }
}
