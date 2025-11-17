pub mod common;
pub mod dense;
pub mod hull;

// INFO: --------------------------------
//         public mesh entrypoint
// --------------------------------------

use crate::prelude::*;
use crate::simulation_world::{
    asset_management::{texture_map_registry::TextureMapResource, MeshAsset},
    block::{
        block_registry::{BlockId, AIR_BLOCK_ID},
        BlockRegistryResource,
    },
    chunk::{chunk_blocks::ChunkView, PaddedChunkView},
};

// convenience mesh types
pub type OpaqueMeshData = MeshAsset;
pub type TransparentMeshData = MeshAsset;

/// Main chunk meshing entry point: Build a mesh for a single chunk.
#[instrument(skip_all)]
pub fn build_chunk_mesh(
    name: &str,
    padded_chunk: PaddedChunkView,
    texture_map: &TextureMapResource,
    block_registry: &BlockRegistryResource,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    let center_view = padded_chunk.get_center_view();
    match center_view {
        // air never has a mesh
        ChunkView::Uniform(block_id) if block_id == AIR_BLOCK_ID => (None, None),
        // solid does hull meshing since it never has internal mesh
        ChunkView::Uniform(block_id) => {
            // fully onccluded by solid neighbors will always be empty
            if is_fully_occluded(&padded_chunk, block_registry, block_id) {
                (None, None)
            } else {
                hull::build_hull_mesh(name, padded_chunk, texture_map, block_registry, block_id)
            }
        }
        // otherwise run the most expensive mesher
        ChunkView::Dense(_) => {
            dense::build_dense_mesh(name, padded_chunk, texture_map, block_registry)
        }
    }
}

/// Helper to check if a chunk is completely hidden (surrounded by solid opaque neighbors).
fn is_fully_occluded(
    padded: &PaddedChunkView,
    registry: &BlockRegistryResource,
    center_id: BlockId,
) -> bool {
    let center_props = registry.get(center_id);

    // if center is transparent can't cull
    if center_props.is_transparent {
        return false;
    }

    // check all 6 neighbors for fully opaque, and if all are then we are hidden.
    padded.is_neighbor_fully_opaque(IVec3::Y, registry)
        && padded.is_neighbor_fully_opaque(IVec3::NEG_Y, registry)
        && padded.is_neighbor_fully_opaque(IVec3::NEG_X, registry)
        && padded.is_neighbor_fully_opaque(IVec3::X, registry)
        && padded.is_neighbor_fully_opaque(IVec3::Z, registry)
        && padded.is_neighbor_fully_opaque(IVec3::NEG_Z, registry)
}
