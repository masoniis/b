use super::{common::*, OpaqueMeshData, TransparentMeshData};
use crate::prelude::*;
use crate::simulation_world::{
    asset_management::texture_map_registry::TextureMapResource,
    block::{block_registry::BlockId, BlockRegistryResource},
    chunk::{PaddedChunkView, CHUNK_SIDE_LENGTH},
};

/// Optimized mesher for uniform solid chunks.
///
/// Only iterates the 6 boundary faces, skipping the interior.
pub fn build_hull_mesh(
    name: &str,
    padded_chunk: PaddedChunkView,
    texture_map: &TextureMapResource,
    block_registry: &BlockRegistryResource,
    block_id: BlockId,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let size = padded_chunk.get_size() as usize;
    let scale = (CHUNK_SIDE_LENGTH / size) as f32;

    let i_size = padded_chunk.get_size();
    let center_lod = padded_chunk.center_lod();
    let all_lods = padded_chunk.neighbor_lods();

    let props = block_registry.get(block_id);
    let is_trans = props.is_transparent;

    macro_rules! mesh_plane {
        ($face_idx:expr, $u_range:expr, $v_range:expr, $pos_fn:expr) => {
            let dir = &FACE_DIRECTIONS[$face_idx];
            if !padded_chunk.is_neighbor_fully_opaque(dir.offset, block_registry) {
                let tex_id = (dir.get_texture)(props);
                let tex_index = texture_map.registry.get(tex_id);

                for u in $u_range {
                    for v in $v_range {
                        let pos = $pos_fn(u, v);
                        let neighbor_pos = pos + dir.offset;
                        let neighbor_id = padded_chunk.get_block(neighbor_pos);
                        let neighbor_props = block_registry.get(neighbor_id);

                        if should_render_face(
                            block_id,
                            is_trans,
                            neighbor_id,
                            neighbor_props.is_transparent,
                        ) {
                            let base_count = vertices.len() as u32;
                            let ao =
                                calculate_ao_for_pos(pos, $face_idx, &padded_chunk, block_registry);

                            let (v, i) = create_face_verts(
                                &dir.face, pos, scale, tex_index, base_count, ao, i_size,
                                center_lod, all_lods,
                            );
                            vertices.extend(v);
                            indices.extend(i);
                        }
                    }
                }
            }
        };
    }

    #[rustfmt::skip]
    mesh_plane!(0, 0..size, 0..size, |x, z| IVec3::new(x as i32, (size-1) as i32, z as i32));
    #[rustfmt::skip]
    mesh_plane!(1, 0..size, 0..size, |x, z| IVec3::new(x as i32, 0, z as i32));
    #[rustfmt::skip]
    mesh_plane!(2, 0..size, 0..size, |y, z| IVec3::new(0, y as i32, z as i32));
    #[rustfmt::skip]
    mesh_plane!(3, 0..size, 0..size, |y, z| IVec3::new((size-1) as i32, y as i32, z as i32));
    #[rustfmt::skip]
    mesh_plane!(4, 0..size, 0..size, |x, y| IVec3::new(x as i32, y as i32, (size-1) as i32));
    #[rustfmt::skip]
    mesh_plane!(5, 0..size, 0..size, |x, y| IVec3::new(x as i32, y as i32, 0));

    let (o_v, o_i, t_v, t_i) = if is_trans {
        (Vec::new(), Vec::new(), vertices, indices)
    } else {
        (vertices, indices, Vec::new(), Vec::new())
    };

    build_mesh_assets(name, o_v, o_i, t_v, t_i)
}
