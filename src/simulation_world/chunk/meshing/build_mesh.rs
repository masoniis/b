use crate::prelude::*;
use crate::render_world::types::{TextureId, WorldVertex};
use crate::simulation_world::{
    asset_management::{texture_map_registry::TextureMapResource, MeshAsset},
    block::{
        block_registry::{BlockId, AIR_BLOCK_ID},
        BlockProperties, BlockRegistryResource,
    },
    chunk::{
        chunk_blocks::ChunkView, types::ChunkLod, NeighborLODs, PaddedChunkView, CHUNK_SIDE_LENGTH,
    },
};

type OpaqueMeshData = MeshAsset;
type TransparentMeshData = MeshAsset;

/// Represents a direction to check for face rendering
struct FaceDirection {
    offset: IVec3,
    face: Face,
    get_texture: fn(&BlockProperties) -> TextureId,
}

const FACE_DIRECTIONS: [FaceDirection; 6] = [
    FaceDirection {
        offset: IVec3::new(0, 1, 0),
        face: Face::Top,
        get_texture: |props| props.textures.top,
    },
    FaceDirection {
        offset: IVec3::new(0, -1, 0),
        face: Face::Bottom,
        get_texture: |props| props.textures.bottom,
    },
    FaceDirection {
        offset: IVec3::new(-1, 0, 0),
        face: Face::Left,
        get_texture: |props| props.textures.left,
    },
    FaceDirection {
        offset: IVec3::new(1, 0, 0),
        face: Face::Right,
        get_texture: |props| props.textures.right,
    },
    FaceDirection {
        offset: IVec3::new(0, 0, 1),
        face: Face::Front,
        get_texture: |props| props.textures.front,
    },
    FaceDirection {
        offset: IVec3::new(0, 0, -1),
        face: Face::Back,
        get_texture: |props| props.textures.back,
    },
];

/// Brightness levels for AO, from 0 (open) to 3 (fully occluded)
const AO_MAPPING: [f32; 4] = [1.0, 0.7, 0.4, 0.2];

/// Occlusion offsets for each vertex (v0, v1, v2, v3) of each face.
///
/// Each vertex checks 3 blocks (side1, side2, corner) to check occlusion.
const AO_OFFSETS: [[[IVec3; 3]; 4]; 6] = [
    // Face::Top (Y+)
    [
        [
            IVec3::new(-1, 1, 0),
            IVec3::new(0, 1, 1),
            IVec3::new(-1, 1, 1),
        ], // front-left
        [
            IVec3::new(1, 1, 0),
            IVec3::new(0, 1, 1),
            IVec3::new(1, 1, 1),
        ], // front-right
        [
            IVec3::new(1, 1, 0),
            IVec3::new(0, 1, -1),
            IVec3::new(1, 1, -1),
        ], // back-right
        [
            IVec3::new(-1, 1, 0),
            IVec3::new(0, 1, -1),
            IVec3::new(-1, 1, -1),
        ], // back-left
    ],
    // Face::Bottom (Y-)
    [
        [
            IVec3::new(-1, -1, 0),
            IVec3::new(0, -1, -1),
            IVec3::new(-1, -1, -1),
        ],
        [
            IVec3::new(1, -1, 0),
            IVec3::new(0, -1, -1),
            IVec3::new(1, -1, -1),
        ],
        [
            IVec3::new(1, -1, 0),
            IVec3::new(0, -1, 1),
            IVec3::new(1, -1, 1),
        ],
        [
            IVec3::new(-1, -1, 0),
            IVec3::new(0, -1, 1),
            IVec3::new(-1, -1, 1),
        ],
    ],
    // Face::Left (X-)
    [
        [
            IVec3::new(-1, -1, 0),
            IVec3::new(-1, 0, -1),
            IVec3::new(-1, -1, -1),
        ],
        [
            IVec3::new(-1, -1, 0),
            IVec3::new(-1, 0, 1),
            IVec3::new(-1, -1, 1),
        ],
        [
            IVec3::new(-1, 1, 0),
            IVec3::new(-1, 0, 1),
            IVec3::new(-1, 1, 1),
        ],
        [
            IVec3::new(-1, 1, 0),
            IVec3::new(-1, 0, -1),
            IVec3::new(-1, 1, -1),
        ],
    ],
    // Face::Right (X+)
    [
        [
            IVec3::new(1, -1, 0),
            IVec3::new(1, 0, 1),
            IVec3::new(1, -1, 1),
        ],
        [
            IVec3::new(1, -1, 0),
            IVec3::new(1, 0, -1),
            IVec3::new(1, -1, -1),
        ],
        [
            IVec3::new(1, 1, 0),
            IVec3::new(1, 0, -1),
            IVec3::new(1, 1, -1),
        ],
        [
            IVec3::new(1, 1, 0),
            IVec3::new(1, 0, 1),
            IVec3::new(1, 1, 1),
        ],
    ],
    // Face::Front (Z+)
    [
        [
            IVec3::new(-1, 0, 1),
            IVec3::new(0, -1, 1),
            IVec3::new(-1, -1, 1),
        ],
        [
            IVec3::new(1, 0, 1),
            IVec3::new(0, -1, 1),
            IVec3::new(1, -1, 1),
        ],
        [
            IVec3::new(1, 0, 1),
            IVec3::new(0, 1, 1),
            IVec3::new(1, 1, 1),
        ],
        [
            IVec3::new(-1, 0, 1),
            IVec3::new(0, 1, 1),
            IVec3::new(-1, 1, 1),
        ],
    ],
    // Face::Back (Z-)
    [
        [
            IVec3::new(1, 0, -1),
            IVec3::new(0, -1, -1),
            IVec3::new(1, -1, -1),
        ],
        [
            IVec3::new(-1, 0, -1),
            IVec3::new(0, -1, -1),
            IVec3::new(-1, -1, -1),
        ],
        [
            IVec3::new(-1, 0, -1),
            IVec3::new(0, 1, -1),
            IVec3::new(-1, 1, -1),
        ],
        [
            IVec3::new(1, 0, -1),
            IVec3::new(0, 1, -1),
            IVec3::new(1, 1, -1),
        ],
    ],
];

/// Determine if a face should be rendered based on transparency rules
#[inline(always)]
fn should_render_face(
    current_id: BlockId,
    current_transparent: bool,
    neighbor_id: BlockId,
    neighbor_transparent: bool,
) -> bool {
    // TODO: if block is on a border and neighbor is UPSAMPLED then we should conservatively
    // always render the face
    match (current_transparent, neighbor_transparent) {
        (false, true) => true,                     // opaque facing transparent
        (true, true) => current_id != neighbor_id, // different transparent blocks
        _ => false,
    }
}

/// Get the AO value (0-3) for a single vertex.
#[inline(always)]
fn get_ao(
    pos: IVec3,
    side1_off: IVec3,
    side2_off: IVec3,
    corner_off: IVec3,
    padded_chunk: &PaddedChunkView,
    block_registry: &BlockRegistryResource,
) -> u8 {
    let s1 = !block_registry
        .get(padded_chunk.get_block(pos + side1_off))
        .is_transparent;
    let s2 = !block_registry
        .get(padded_chunk.get_block(pos + side2_off))
        .is_transparent;
    let c = !block_registry
        .get(padded_chunk.get_block(pos + corner_off))
        .is_transparent;

    if s1 && s2 {
        3 // max occlusion if both sides are blocked
    } else {
        (s1 as u8) + (s2 as u8) + (c as u8)
    }
}

/// Helper to check if a neighbor chunk is completely hidden (Solid Opaque).
fn is_fully_occluded(
    padded: &PaddedChunkView,
    registry: &BlockRegistryResource,
    center_id: BlockId,
) -> bool {
    let center_props = registry.get(center_id);

    // If center is transparent (e.g. Water), we can't cull just because neighbors are solid.
    if center_props.is_transparent {
        return false;
    }

    // Check all 6 faces. If all neighbors are fully opaque, we are hidden.
    padded.is_neighbor_fully_opaque(IVec3::Y, registry)
        && padded.is_neighbor_fully_opaque(IVec3::NEG_Y, registry)
        && padded.is_neighbor_fully_opaque(IVec3::NEG_X, registry)
        && padded.is_neighbor_fully_opaque(IVec3::X, registry)
        && padded.is_neighbor_fully_opaque(IVec3::Z, registry)
        && padded.is_neighbor_fully_opaque(IVec3::NEG_Z, registry)
}

/// Main entry point: Build a mesh for a single chunk, filtering uniform cases.
#[instrument(skip_all)]
pub fn build_chunk_mesh(
    name: &str,
    padded_chunk: PaddedChunkView,
    texture_map: &TextureMapResource,
    block_registry: &BlockRegistryResource,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    // 1. Get Center View
    let center_view = padded_chunk.get_center_view();

    match center_view {
        // CASE: Uniform Air
        // Optimization: Instant Exit (0 cycles)
        ChunkView::Uniform(block_id) if block_id == AIR_BLOCK_ID => (None, None),

        // CASE: Uniform Solid
        // Optimization: Hull Meshing
        ChunkView::Uniform(block_id) => {
            // Check if we are completely buried (occluded by neighbors)
            if is_fully_occluded(&padded_chunk, block_registry, block_id) {
                // Buried deep underground. Instant Exit.
                (None, None)
            } else {
                // Touching air/transparent blocks. Run the Hull Mesher.
                build_hull_mesh(name, padded_chunk, texture_map, block_registry, block_id)
            }
        }

        // CASE: Dense (Mixed)
        // Fallback: Run the standard Voxel Mesher
        ChunkView::Dense(_) => build_dense_mesh(name, padded_chunk, texture_map, block_registry),
    }
}

/// Standard mesher for dense, mixed-block chunks.
fn build_dense_mesh(
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

    // get LOD info for visual stitching
    let i_size = padded_chunk.get_size();
    let center_lod = padded_chunk.center_lod();
    let all_lods = padded_chunk.neighbor_lods();

    for x in 0..size {
        for z in 0..size {
            for y in 0..size {
                let pos = IVec3::new(x as i32, y as i32, z as i32);
                let current_block_id = padded_chunk.get_block(pos);

                // skip air blocks
                if current_block_id == AIR_BLOCK_ID {
                    continue;
                }

                // get block properties
                let current_block_props = block_registry.get(current_block_id);
                let (target_vertices, target_indices) = if current_block_props.is_transparent {
                    (&mut transparent_vertices, &mut transparent_indices)
                } else {
                    (&mut opaque_vertices, &mut opaque_indices)
                };

                // check all 6 faces
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

                        // calculate ao
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

/// Optimized mesher for uniform solid chunks (e.g. Cave walls).
/// Only iterates the 6 boundary faces, skipping the interior.
fn build_hull_mesh(
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

    // Helper macro to iterate a 2D plane
    macro_rules! mesh_plane {
        ($face_idx:expr, $u_range:expr, $v_range:expr, $pos_fn:expr) => {
            // Optimization: Skip this face if the neighbor is fully opaque
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

    // Generate the 6 faces
    mesh_plane!(0, 0..size, 0..size, |x, z| IVec3::new(
        x as i32,
        (size - 1) as i32,
        z as i32
    )); // Top
    mesh_plane!(1, 0..size, 0..size, |x, z| IVec3::new(
        x as i32, 0, z as i32
    )); // Bottom
    mesh_plane!(2, 0..size, 0..size, |y, z| IVec3::new(
        0, y as i32, z as i32
    )); // Left
    mesh_plane!(3, 0..size, 0..size, |y, z| IVec3::new(
        (size - 1) as i32,
        y as i32,
        z as i32
    )); // Right
    mesh_plane!(4, 0..size, 0..size, |x, y| IVec3::new(
        x as i32,
        y as i32,
        (size - 1) as i32
    )); // Front
    mesh_plane!(5, 0..size, 0..size, |x, y| IVec3::new(
        x as i32, y as i32, 0
    )); // Back

    // Return result
    let (opaque_verts, opaque_inds, trans_verts, trans_inds) = if is_trans {
        // If transparent, the generated vertices go into the TRANSPARENT buckets
        (Vec::new(), Vec::new(), vertices, indices)
    } else {
        // If opaque, they go into the OPAQUE buckets
        (vertices, indices, Vec::new(), Vec::new())
    };

    build_mesh_assets(name, opaque_verts, opaque_inds, trans_verts, trans_inds)
}

// --- Helpers ---

fn calculate_ao_for_pos(
    pos: IVec3,
    face_idx: usize,
    padded_chunk: &PaddedChunkView,
    block_registry: &BlockRegistryResource,
) -> [f32; 4] {
    [
        AO_MAPPING[get_ao(
            pos,
            AO_OFFSETS[face_idx][0][0],
            AO_OFFSETS[face_idx][0][1],
            AO_OFFSETS[face_idx][0][2],
            padded_chunk,
            block_registry,
        ) as usize],
        AO_MAPPING[get_ao(
            pos,
            AO_OFFSETS[face_idx][1][0],
            AO_OFFSETS[face_idx][1][1],
            AO_OFFSETS[face_idx][1][2],
            padded_chunk,
            block_registry,
        ) as usize],
        AO_MAPPING[get_ao(
            pos,
            AO_OFFSETS[face_idx][2][0],
            AO_OFFSETS[face_idx][2][1],
            AO_OFFSETS[face_idx][2][2],
            padded_chunk,
            block_registry,
        ) as usize],
        AO_MAPPING[get_ao(
            pos,
            AO_OFFSETS[face_idx][3][0],
            AO_OFFSETS[face_idx][3][1],
            AO_OFFSETS[face_idx][3][2],
            padded_chunk,
            block_registry,
        ) as usize],
    ]
}

fn build_mesh_assets(
    name: &str,
    ov: Vec<WorldVertex>,
    oi: Vec<u32>,
    tv: Vec<WorldVertex>,
    ti: Vec<u32>,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    let opaque = if !ov.is_empty() {
        Some(OpaqueMeshData {
            name: name.to_string(),
            vertices: ov,
            indices: oi,
        })
    } else {
        None
    };

    let trans = if !tv.is_empty() {
        Some(TransparentMeshData {
            name: format!("{}_trans", name),
            vertices: tv,
            indices: ti,
        })
    } else {
        None
    };

    (opaque, trans)
}

#[derive(Clone, Copy)]
enum Face {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
    Front = 4,
    Back = 5,
}

/// Vertex offsets for each corner of a block's face.
/// Used to determine which chunks share a given vertex.
const VERTEX_OFFSETS: [[IVec3; 4]; 6] = [
    // Top
    [
        IVec3::new(-1, 1, 1),  // v0 (front-left)
        IVec3::new(1, 1, 1),   // v1 (front-right)
        IVec3::new(1, 1, -1),  // v2 (back-right)
        IVec3::new(-1, 1, -1), // v3 (back-left)
    ],
    // Bottom
    [
        IVec3::new(-1, -1, -1),
        IVec3::new(1, -1, -1),
        IVec3::new(1, -1, 1),
        IVec3::new(-1, -1, 1),
    ],
    // Left
    [
        IVec3::new(-1, -1, -1),
        IVec3::new(-1, -1, 1),
        IVec3::new(-1, 1, 1),
        IVec3::new(-1, 1, -1),
    ],
    // Right
    [
        IVec3::new(1, -1, 1),
        IVec3::new(1, -1, -1),
        IVec3::new(1, 1, -1),
        IVec3::new(1, 1, 1),
    ],
    // Front
    [
        IVec3::new(-1, -1, 1),
        IVec3::new(1, -1, 1),
        IVec3::new(1, 1, 1),
        IVec3::new(-1, 1, 1),
    ],
    // Back
    [
        IVec3::new(1, -1, -1),
        IVec3::new(-1, -1, -1),
        IVec3::new(-1, 1, -1),
        IVec3::new(1, 1, -1),
    ],
];

/// Calculates the snapped position of a vertex based on the max LOD of all chunks sharing it.
fn get_snapped_pos(
    vertex_world_pos: [f32; 3],
    vertex_offset: IVec3, // vertex's offset from the block center (e.g., (1, 1, 1))
    block_chunk_pos: IVec3, // block's position within the chunk (0-31)
    chunk_size: i32,
    center_lod: ChunkLod,
    all_lods: &NeighborLODs,
) -> [f32; 3] {
    let max_idx = chunk_size - 1;

    // INFO: -----------------------------------------------
    //         determine if we are on chunk boundary
    // -----------------------------------------------------

    let x_check_idx = if block_chunk_pos.x == 0 && vertex_offset.x < 0 {
        0 // on -X boundary
    } else if block_chunk_pos.x == max_idx && vertex_offset.x > 0 {
        2 // on +X boundary
    } else {
        1 // not on X boundary
    };

    let y_check_idx = if block_chunk_pos.y == 0 && vertex_offset.y < 0 {
        0 // On -Y boundary
    } else if block_chunk_pos.y == max_idx && vertex_offset.y > 0 {
        2 // on +Y boundary
    } else {
        1 // not on Y boundary
    };

    let z_check_idx = if block_chunk_pos.z == 0 && vertex_offset.z < 0 {
        0 // on -Z boundary
    } else if block_chunk_pos.z == max_idx && vertex_offset.z > 0 {
        2 // on +Z boundary
    } else {
        1 // not on Z boundary
    };

    // INFO: ------------------------------------------------
    //         find highest LOD of all sharing chunks
    // ------------------------------------------------------

    let (x_min, x_max) = if x_check_idx != 1 {
        (x_check_idx.min(1), x_check_idx.max(1))
    } else {
        (1, 1)
    };
    let (y_min, y_max) = if y_check_idx != 1 {
        (y_check_idx.min(1), y_check_idx.max(1))
    } else {
        (1, 1)
    };
    let (z_min, z_max) = if z_check_idx != 1 {
        (z_check_idx.min(1), z_check_idx.max(1))
    } else {
        (1, 1)
    };

    let mut max_lod = center_lod;
    for i in x_min..=x_max {
        for j in y_min..=y_max {
            for k in z_min..=z_max {
                if let Some(lod) = all_lods[i as usize][j as usize][k as usize] {
                    max_lod = max_lod.max(lod);
                }
            }
        }
    }

    // snap to max lod grid
    if max_lod > center_lod {
        let grid_scale = (1 << *max_lod) as f32;
        let snap = |c: f32| (c / grid_scale).round() * grid_scale;
        [
            snap(vertex_world_pos[0]),
            snap(vertex_world_pos[1]),
            snap(vertex_world_pos[2]),
        ]
    } else {
        vertex_world_pos // no snapping
    }
}

fn create_face_verts(
    face: &Face,
    block_pos: IVec3,
    scale: f32,
    tex_index: u32,
    base_vertex_count: u32,
    ao_values: [f32; 4],
    size: i32,
    center_lod: ChunkLod,
    all_lods: &NeighborLODs,
) -> (Vec<WorldVertex>, [u32; 6]) {
    let half_s = scale * 0.5;
    let (fx, fy, fz) = (
        (block_pos.x as f32 * scale) + half_s,
        (block_pos.y as f32 * scale) + half_s,
        (block_pos.z as f32 * scale) + half_s,
    );

    let (verts, normal): (Vec<[f32; 3]>, [f32; 3]) = match face {
        Face::Top => (
            vec![
                [-half_s + fx, half_s + fy, half_s + fz],  // v0
                [half_s + fx, half_s + fy, half_s + fz],   // v1
                [half_s + fx, half_s + fy, -half_s + fz],  // v2
                [-half_s + fx, half_s + fy, -half_s + fz], // v3
            ],
            [0.0, 1.0, 0.0],
        ),
        Face::Bottom => (
            vec![
                [-half_s + fx, -half_s + fy, -half_s + fz],
                [half_s + fx, -half_s + fy, -half_s + fz],
                [half_s + fx, -half_s + fy, half_s + fz],
                [-half_s + fx, -half_s + fy, half_s + fz],
            ],
            [0.0, -1.0, 0.0],
        ),
        Face::Left => (
            vec![
                [-half_s + fx, -half_s + fy, -half_s + fz],
                [-half_s + fx, -half_s + fy, half_s + fz],
                [-half_s + fx, half_s + fy, half_s + fz],
                [-half_s + fx, half_s + fy, -half_s + fz],
            ],
            [-1.0, 0.0, 0.0],
        ),
        Face::Right => (
            vec![
                [half_s + fx, -half_s + fy, half_s + fz],
                [half_s + fx, -half_s + fy, -half_s + fz],
                [half_s + fx, half_s + fy, -half_s + fz],
                [half_s + fx, half_s + fy, half_s + fz],
            ],
            [1.0, 0.0, 0.0],
        ),
        Face::Front => (
            vec![
                [-half_s + fx, -half_s + fy, half_s + fz],
                [half_s + fx, -half_s + fy, half_s + fz],
                [half_s + fx, half_s + fy, half_s + fz],
                [-half_s + fx, half_s + fy, half_s + fz],
            ],
            [0.0, 0.0, 1.0],
        ),
        Face::Back => (
            vec![
                [half_s + fx, -half_s + fy, -half_s + fz],
                [-half_s + fx, -half_s + fy, -half_s + fz],
                [-half_s + fx, half_s + fy, -half_s + fz],
                [half_s + fx, half_s + fy, -half_s + fz],
            ],
            [0.0, 0.0, -1.0],
        ),
    };

    // Get the vertex offsets for the current face
    let face_idx = *face as usize;
    let v_offsets = &VERTEX_OFFSETS[face_idx];

    // Calculate final snapped positions
    let snapped_verts = [
        get_snapped_pos(
            verts[0],
            v_offsets[0],
            block_pos,
            size,
            center_lod,
            all_lods,
        ),
        get_snapped_pos(
            verts[1],
            v_offsets[1],
            block_pos,
            size,
            center_lod,
            all_lods,
        ),
        get_snapped_pos(
            verts[2],
            v_offsets[2],
            block_pos,
            size,
            center_lod,
            all_lods,
        ),
        get_snapped_pos(
            verts[3],
            v_offsets[3],
            block_pos,
            size,
            center_lod,
            all_lods,
        ),
    ];

    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    // apply ao to color for the 4 vertices
    let final_vertices = vec![
        WorldVertex::new(
            snapped_verts[0],
            normal,
            [ao_values[0], ao_values[0], ao_values[0]],
            uvs[0],
            tex_index,
        ),
        WorldVertex::new(
            snapped_verts[1],
            normal,
            [ao_values[1], ao_values[1], ao_values[1]],
            uvs[1],
            tex_index,
        ),
        WorldVertex::new(
            snapped_verts[2],
            normal,
            [ao_values[2], ao_values[2], ao_values[2]],
            uvs[2],
            tex_index,
        ),
        WorldVertex::new(
            snapped_verts[3],
            normal,
            [ao_values[3], ao_values[3], ao_values[3]],
            uvs[3],
            tex_index,
        ),
    ];

    // ao diagonal determines triangle split
    let indices = if (ao_values[0] + ao_values[2]) > (ao_values[1] + ao_values[3]) {
        [
            base_vertex_count + 0,
            base_vertex_count + 1,
            base_vertex_count + 2,
            base_vertex_count + 2,
            base_vertex_count + 3,
            base_vertex_count + 0,
        ]
    } else {
        [
            base_vertex_count + 0,
            base_vertex_count + 1,
            base_vertex_count + 3,
            base_vertex_count + 1,
            base_vertex_count + 2,
            base_vertex_count + 3,
        ]
    };

    (final_vertices, indices)
}
