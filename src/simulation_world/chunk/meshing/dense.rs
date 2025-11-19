use super::{common::*, OpaqueMeshData, TransparentMeshData};
use crate::prelude::*;
use crate::simulation_world::{
    asset_management::texture_map_registry::TextureMapResource,
    block::{block_registry::AIR_BLOCK_ID, BlockRegistryResource},
    chunk::{PaddedChunkView, CHUNK_SIDE_LENGTH},
};

/// Standard mesher for dense, mixed-block chunks.
#[instrument(skip_all)]
pub fn build_dense_mesh(
    name: &str,
    padded_chunk: PaddedChunkView,
    texture_map: &TextureMapResource,
    block_registry: &BlockRegistryResource,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    let mut opaque_vertices = Vec::new();
    let mut opaque_indices = Vec::new();
    let mut transparent_vertices = Vec::new();
    let mut transparent_indices = Vec::new();

    let size = padded_chunk.get_size() as usize;
    let scale = (CHUNK_SIDE_LENGTH / size) as f32;
    let i_size = padded_chunk.get_size();
    let center_lod = padded_chunk.center_lod();
    let all_lods = padded_chunk.neighbor_lods();

    for x in 0..size {
        for z in 0..size {
            for y in 0..size {
                let pos = IVec3::new(x as i32, y as i32, z as i32);
                let current_block_id = padded_chunk.get_block(pos);

                if current_block_id == AIR_BLOCK_ID {
                    continue;
                }

                let current_block_props = block_registry.get(current_block_id);
                let (target_vertices, target_indices) = if current_block_props.is_transparent {
                    (&mut transparent_vertices, &mut transparent_indices)
                } else {
                    (&mut opaque_vertices, &mut opaque_indices)
                };

                for (face_index, direction) in FACE_DIRECTIONS.iter().enumerate() {
                    let neighbor_pos = pos + direction.offset;
                    let neighbor_id = padded_chunk.get_block(neighbor_pos);
                    let neighbor_props = block_registry.get(neighbor_id);

                    if should_render_face(
                        current_block_id,
                        current_block_props.is_transparent,
                        neighbor_id,
                        neighbor_props.is_transparent,
                    ) {
                        let base_vertex_count = target_vertices.len() as u32;
                        let tex_id = (direction.get_texture)(current_block_props);
                        let tex_index = texture_map.registry.get(tex_id);

                        let ao_values =
                            calculate_ao_for_pos(pos, face_index, &padded_chunk, block_registry);

                        let (face_verts, face_indices) = create_face_verts(
                            &direction.face,
                            pos,
                            scale,
                            tex_index,
                            base_vertex_count,
                            ao_values,
                            i_size,
                            center_lod,
                            all_lods,
                        );
                        target_vertices.extend(face_verts);
                        target_indices.extend(face_indices);
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
