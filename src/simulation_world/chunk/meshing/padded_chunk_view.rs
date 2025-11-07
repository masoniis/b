use crate::prelude::*;
use crate::simulation_world::{
    block::block_registry::{BlockId, AIR_BLOCK_ID},
    chunk::{types::ChunkLod, ChunkBlocksComponent},
};

/// Holds the *original* LODs of all 26 neighbors (and the center)
/// in a 3x3x3 grid. [1][1][1] is the center.
///
/// Used for visual seam stitching.
pub type NeighborLODs = [[[Option<ChunkLod>; 3]; 3]; 3];

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

    /// The size of one edge of the *center* chunk (e.g., 32, 16, 8).
    size: i32,

    /// The LOD of the center (main padded) chunk
    center_lod: ChunkLod,
    /// The original LOD of the 6 cardinal neighbors before scaling to match the center LOD.
    neighbor_lods: NeighborLODs,
}

impl PaddedChunkView {
    /// Creates a new view for a specific chunk of arbitrary LOD.
    pub fn new(chunks: [[[ChunkDataOption; 3]; 3]; 3], neighbor_lods: NeighborLODs) -> Self {
        let (size, center_lod) = match &chunks[1][1][1] {
            ChunkDataOption::Generated(center_chunk) => {
                (center_chunk.size() as i32, center_chunk.lod())
            }
            _ => {
                panic!("PaddedChunkView::new: Center chunk must be `Generated`.")
            }
        };

        if cfg!(debug_assertions) {
            for chunk_data_opt in chunks
                .iter()
                .flat_map(|plane| plane.iter().flat_map(|row| row.iter()))
            {
                if let ChunkDataOption::Generated(chunk) = chunk_data_opt {
                    let neighbor_size = chunk.size() as i32;
                    if neighbor_size != size {
                        panic!(
                            "PaddedChunkView::new: Inconsistent LODs. Center chunk size is {}, but found neighbor with size {}. All chunks must be pre-stitched to the same size.",
                            size, neighbor_size
                        );
                    }
                }
            }
        }

        Self {
            chunks,
            size,
            neighbor_lods,
            center_lod,
        }
    }

    /// Gets the size of one edge of the center chunk.
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Gets the LOD of the center chunk.
    pub fn center_lod(&self) -> ChunkLod {
        self.center_lod
    }

    /// Gets the original LODs of the 6 cardinal neighbors.
    pub fn neighbor_lods(&self) -> &NeighborLODs {
        &self.neighbor_lods
    }

    /// Gets a block ID from the padded view. A value higher or lower than
    /// the chunk dimensions will access neighboring chunks.
    ///
    /// WARN: Assumes valid input for runtime efficiency, but if you exit
    /// the neighbor chunk undefined behavior may occur.
    pub fn get_block(&self, pos: IVec3) -> BlockId {
        let w = self.size;
        let h = self.size;
        let d = self.size;

        if cfg!(debug_assertions)
            && (pos.x < -w
                || pos.x >= 2 * w
                || pos.y < -h
                || pos.y >= 2 * h
                || pos.z < -d
                || pos.z >= 2 * d)
        {
            error!(
                "get_block: Attempted to access block out of padded chunk bounds: ({}, {}, {})",
                pos.x, pos.y, pos.z
            );
        }

        // determine chunk to read from (0, 1, or 2)
        let chunk_idx_x = (pos.x.div_euclid(w) + 1) as usize;
        let chunk_idx_y = (pos.y.div_euclid(h) + 1) as usize;
        let chunk_idx_z = (pos.z.div_euclid(d) + 1) as usize;

        match &self.chunks[chunk_idx_x][chunk_idx_y][chunk_idx_z] {
            ChunkDataOption::Generated(chunk) => {
                let local_x = pos.x.rem_euclid(w) as usize;
                let local_y = pos.y.rem_euclid(h) as usize;
                let local_z = pos.z.rem_euclid(d) as usize;

                chunk.get_data_unchecked(local_x, local_y, local_z)
            }
            ChunkDataOption::OutOfBounds => 1, // solid block
            ChunkDataOption::Empty => AIR_BLOCK_ID,
        }
    }
}
