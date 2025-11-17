use crate::prelude::*;
use crate::simulation_world::block::{BlockRegistryResource, SOLID_BLOCK_ID};
use crate::simulation_world::{
    block::block_registry::{BlockId, AIR_BLOCK_ID},
    chunk::{chunk_blocks::ChunkView, types::ChunkLod, ChunkBlocksComponent},
};

/// Holds the *original* LODs of all 26 neighbors (and the center)
/// in a 3x3x3 grid. [1][1][1] is the center.
///
/// Needed for visual seam stitching of mismatched LOD chunks.
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
pub struct PaddedChunkView<'a> {
    /// A 3x3x3 grid of lightweight views.
    /// Index [1][1][1] is the center chunk.
    views: [[[ChunkView<'a>; 3]; 3]; 3],

    /// The size of one edge of the *center* chunk (e.g., 32).
    size: i32,

    /// `size - 1`. Used for fast modulo (x & mask).
    mask: i32,
    /// `log2(size)`. Used for fast division (x >> shift).
    bit_shift: u32,

    center_lod: ChunkLod,
    neighbor_lods: NeighborLODs,
}

impl<'a> PaddedChunkView<'a> {
    /// Creates a new padded view for a chunk.
    pub fn new(chunks: &'a [[[ChunkDataOption; 3]; 3]; 3], neighbor_lods: NeighborLODs) -> Self {
        // extract size/lod from center chunk
        let (size, center_lod) = match &chunks[1][1][1] {
            ChunkDataOption::Generated(center_chunk) => {
                (center_chunk.size() as i32, center_chunk.lod())
            }
            _ => panic!("PaddedChunkView::new: Center chunk must be `Generated`."),
        };

        // prepare individual chunk views
        let mut views = [[[ChunkView::Uniform(AIR_BLOCK_ID); 3]; 3]; 3];
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    views[x][y][z] = match &chunks[x][y][z] {
                        ChunkDataOption::Generated(comp) => comp.get_view(),
                        ChunkDataOption::Empty => ChunkView::Uniform(AIR_BLOCK_ID),
                        ChunkDataOption::OutOfBounds => ChunkView::Uniform(SOLID_BLOCK_ID),
                    };
                }
            }
        }

        // precompute constants for chunk determinance
        let mask = size - 1;
        let bit_shift = (size as u32).trailing_zeros();

        Self {
            views,
            size,
            mask,
            bit_shift,
            center_lod,
            neighbor_lods,
        }
    }

    /// Helper to get the raw view of the center chunk.
    pub fn get_center_view(&self) -> ChunkView<'_> {
        unsafe {
            self.views
                .get_unchecked(1)
                .get_unchecked(1)
                .get_unchecked(1)
                .clone()
        }
    }

    /// Gets the size of one edge of the center chunk.
    pub fn get_size(&self) -> i32 {
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

    /// Checks if a neighbor is fully solid/opaque (Uniform Solid).
    pub fn is_neighbor_fully_opaque(
        &self,
        offset: IVec3,
        block_registry: &BlockRegistryResource,
    ) -> bool {
        let nx = (offset.x + 1) as usize;
        let ny = (offset.y + 1) as usize;
        let nz = (offset.z + 1) as usize;

        let view = unsafe {
            self.views
                .get_unchecked(nx)
                .get_unchecked(ny)
                .get_unchecked(nz)
        };

        match view {
            ChunkView::Uniform(block_id) => {
                let props = block_registry.get(*block_id);
                !props.is_transparent && *block_id != AIR_BLOCK_ID
            }
            // if neighbor is dense there is no easy way to check full opaque without scanning
            ChunkView::Dense(_) => false,
        }
    }

    /// Gets a block ID from the padded view.
    ///
    /// WARN: Assumes valid input for runtime efficiency, but if you exit
    /// the neighbor chunk undefined behavior will occur.
    #[inline(always)]
    pub fn get_block(&self, pos: IVec3) -> BlockId {
        // determine chunk index (0, 1, 2)
        let cx = ((pos.x >> self.bit_shift) + 1) as usize;
        let cy = ((pos.y >> self.bit_shift) + 1) as usize;
        let cz = ((pos.z >> self.bit_shift) + 1) as usize;

        if cfg!(debug_assertions) && (cx > 2 || cy > 2 || cz > 2) {
            error!(
                "get_block: Out of bounds: ({}, {}, {})",
                pos.x, pos.y, pos.z
            );
            return AIR_BLOCK_ID;
        }

        // determine local index
        let lx = (pos.x & self.mask) as usize;
        let ly = (pos.y & self.mask) as usize;
        let lz = (pos.z & self.mask) as usize;

        // fetch from view
        let view = unsafe {
            self.views
                .get_unchecked(cx)
                .get_unchecked(cy)
                .get_unchecked(cz)
        };

        match view {
            ChunkView::Uniform(id) => *id,
            ChunkView::Dense(vol) => vol.get_data(lx, ly, lz),
        }
    }
}
