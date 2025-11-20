use super::{common::*, OpaqueMeshData, TransparentMeshData};
use crate::prelude::*;
use crate::render_world::textures::registry::TextureRegistryResource;
use crate::simulation_world::{
    block::{block_registry::AIR_BLOCK_ID, BlockRegistryResource},
    chunk::{PaddedChunk, CHUNK_SIDE_LENGTH},
};

/// Standard mesher for dense, mixed-block chunks.
#[instrument(skip_all)]
pub fn build_dense_mesh(
    name: &str,
    padded_chunk: &PaddedChunk,
    texture_map: &TextureRegistryResource,
    block_registry: &BlockRegistryResource,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    // TODO: using a buffer pool is probably better than this alloc guesswork
    // even though we ultimately need a personal copy of the data at the end
    let mut opaque_vertices = Vec::with_capacity(20_000);
    let mut opaque_indices = Vec::with_capacity(30_000);
    let mut transparent_vertices = Vec::with_capacity(5_000);
    let mut transparent_indices = Vec::with_capacity(7_500);

    let ctx = MesherContext {
        padded_chunk,
        block_registry,
        texture_map,
        center_lod: padded_chunk.center_lod(),
        neighbor_lods: padded_chunk.neighbor_lods(),
        chunk_size: padded_chunk.get_size(),
        scale: (CHUNK_SIDE_LENGTH / padded_chunk.get_size()) as f32,
    };

    let size = ctx.chunk_size;

    for x in 0..size {
        for z in 0..size {
            for y in 0..size {
                let pos = IVec3::new(x as i32, y as i32, z as i32);
                let current_block_id = padded_chunk.get_block(pos.x, pos.y, pos.z);

                if current_block_id == AIR_BLOCK_ID {
                    continue;
                }

                let props = block_registry.get(current_block_id);

                let (verts, idxs) = if props.is_transparent {
                    (&mut transparent_vertices, &mut transparent_indices)
                } else {
                    (&mut opaque_vertices, &mut opaque_indices)
                };

                // iterate each face checking and generating face verts
                for face_i in 0..6 {
                    let offset = NEIGHBOR_OFFSETS[face_i];
                    let neighbor_pos = pos + offset;
                    let neighbor_id =
                        padded_chunk.get_block(neighbor_pos.x, neighbor_pos.y, neighbor_pos.z);
                    let neighbor_props = block_registry.get(neighbor_id);

                    if should_render_face(
                        current_block_id,
                        props.is_transparent,
                        neighbor_id,
                        neighbor_props.is_transparent,
                    ) {
                        let tex_id = props.textures.get(face_i);
                        let ao = calculate_ao_for_pos(pos, face_i, &padded_chunk, block_registry);
                        ctx.push_face(face_i, pos, tex_id, ao, verts, idxs);
                    }
                }
            }
        }
    }

    build_mesh_assets(
        name,
        opaque_vertices,
        opaque_indices,
        transparent_vertices,
        transparent_indices,
    )
}
