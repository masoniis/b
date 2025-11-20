use super::{common::*, OpaqueMeshData, TransparentMeshData};
use crate::prelude::*;
use crate::render_world::textures::registry::TextureRegistryResource;
use crate::simulation_world::{
    block::{block_registry::BlockId, BlockRegistryResource},
    chunk::{PaddedChunk, CHUNK_SIDE_LENGTH},
};

/// Optimized mesher for uniform solid chunks.
///
/// Only iterates the 6 boundary faces, skipping the interior.
#[instrument(skip_all)]
pub fn build_hull_mesh(
    name: &str,
    padded_chunk: &PaddedChunk,
    texture_map: &TextureRegistryResource,
    block_registry: &BlockRegistryResource,
    block_id: BlockId,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    let mut vertices = Vec::with_capacity(4096);
    let mut indices = Vec::with_capacity(6144);

    let ctx = MesherContext {
        padded_chunk,
        block_registry,
        texture_map,
        center_lod: padded_chunk.center_lod(),
        neighbor_lods: padded_chunk.neighbor_lods(),
        chunk_size: padded_chunk.get_size(),
        scale: CHUNK_SIDE_LENGTH as f32 / padded_chunk.get_size() as f32,
    };

    let size = ctx.chunk_size;
    let props = block_registry.get(block_id);
    let is_trans = props.is_transparent;

    macro_rules! mesh_plane {
        ($face_idx:expr, $u_range:expr, $v_range:expr, $pos_fn:expr) => {
            let offset = NEIGHBOR_OFFSETS[$face_idx];

            if !ctx
                .padded_chunk
                .is_neighbor_fully_opaque(offset, ctx.block_registry)
            {
                let tex_id = match $face_idx {
                    0 => props.textures.top,
                    1 => props.textures.bottom,
                    2 => props.textures.left,
                    3 => props.textures.right,
                    4 => props.textures.front,
                    _ => props.textures.back,
                };

                for u in $u_range {
                    for v in $v_range {
                        let pos = $pos_fn(u, v);

                        let neighbor_pos = pos + offset;
                        let neighbor_id = ctx.padded_chunk.get_block(
                            neighbor_pos.x,
                            neighbor_pos.y,
                            neighbor_pos.z,
                        );
                        let neighbor_props = ctx.block_registry.get(neighbor_id);

                        if should_render_face(
                            block_id,
                            is_trans,
                            neighbor_id,
                            neighbor_props.is_transparent,
                        ) {
                            let ao = calculate_ao_for_pos(
                                pos,
                                $face_idx,
                                ctx.padded_chunk,
                                ctx.block_registry,
                            );

                            ctx.push_face($face_idx, pos, tex_id, ao, &mut vertices, &mut indices);
                        }
                    }
                }
            }
        };
    }

    #[rustfmt::skip]
    mesh_plane!(0, 0..size, 0..size, |x, z| IVec3::new(x as i32, (size - 1) as i32, z as i32));
    #[rustfmt::skip]
    mesh_plane!(1, 0..size, 0..size, |x, z| IVec3::new(x as i32, 0, z as i32));
    #[rustfmt::skip]
    mesh_plane!(2, 0..size, 0..size, |y, z| IVec3::new(0, y as i32, z as i32));
    #[rustfmt::skip]
    mesh_plane!(3, 0..size, 0..size, |y, z| IVec3::new((size - 1) as i32, y as i32, z as i32));
    #[rustfmt::skip]
    mesh_plane!(4, 0..size, 0..size, |x, y| IVec3::new(x as i32, y as i32, (size - 1) as i32));
    #[rustfmt::skip]
    mesh_plane!(5, 0..size, 0..size, |x, y| IVec3::new(x as i32, y as i32, 0));

    let (o_v, o_i, t_v, t_i) = if is_trans {
        (Vec::new(), Vec::new(), vertices, indices)
    } else {
        (vertices, indices, Vec::new(), Vec::new())
    };

    build_mesh_assets(name, o_v, o_i, t_v, t_i)
}
