use super::{OpaqueMeshData, TransparentMeshData};
use crate::prelude::*;
use crate::render_world::types::{TextureId, WorldVertex};
use crate::simulation_world::{
    block::{BlockId, BlockProperties, BlockRegistryResource},
    chunk::{types::ChunkLod, NeighborLODs, PaddedChunkView},
};

#[derive(Clone, Copy)]
pub enum Face {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
    Front = 4,
    Back = 5,
}

/// Represents a direction to check for face rendering
pub struct FaceDirection {
    pub offset: IVec3,
    pub face: Face,
    pub get_texture: fn(&BlockProperties) -> TextureId,
}

pub const FACE_DIRECTIONS: [FaceDirection; 6] = [
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
pub const AO_MAPPING: [f32; 4] = [1.0, 0.7, 0.4, 0.2];

/// Occlusion offsets for each vertex.
pub const AO_OFFSETS: [[[IVec3; 3]; 4]; 6] = [
    // Face::Top (Y+)
    [
        [
            IVec3::new(-1, 1, 0),
            IVec3::new(0, 1, 1),
            IVec3::new(-1, 1, 1),
        ],
        [
            IVec3::new(1, 1, 0),
            IVec3::new(0, 1, 1),
            IVec3::new(1, 1, 1),
        ],
        [
            IVec3::new(1, 1, 0),
            IVec3::new(0, 1, -1),
            IVec3::new(1, 1, -1),
        ],
        [
            IVec3::new(-1, 1, 0),
            IVec3::new(0, 1, -1),
            IVec3::new(-1, 1, -1),
        ],
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

pub const VERTEX_OFFSETS: [[IVec3; 4]; 6] = [
    // Top
    [
        IVec3::new(-1, 1, 1),
        IVec3::new(1, 1, 1),
        IVec3::new(1, 1, -1),
        IVec3::new(-1, 1, -1),
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

/// Determine if a face should be rendered based on transparency rules
#[inline(always)]
pub fn should_render_face(
    current_id: BlockId,
    current_transparent: bool,
    neighbor_id: BlockId,
    neighbor_transparent: bool,
) -> bool {
    match (current_transparent, neighbor_transparent) {
        (false, true) => true,                     // opaque facing transparent
        (true, true) => current_id != neighbor_id, // different transparent blocks
        _ => false,
    }
}

/// Get the AO value (0-3) for a single vertex.
#[inline(always)]
pub fn get_ao(
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
        3
    } else {
        (s1 as u8) + (s2 as u8) + (c as u8)
    }
}

/// Calculates the snapped position of a vertex based on the max LOD of all chunks sharing it.
#[instrument(skip_all)]
pub fn get_snapped_pos(
    vertex_world_pos: [f32; 3],
    vertex_offset: IVec3,
    block_chunk_pos: IVec3,
    chunk_size: i32,
    center_lod: ChunkLod,
    all_lods: &NeighborLODs,
) -> [f32; 3] {
    let max_idx = chunk_size - 1;
    let x_check_idx = if block_chunk_pos.x == 0 && vertex_offset.x < 0 {
        0
    } else if block_chunk_pos.x == max_idx && vertex_offset.x > 0 {
        2
    } else {
        1
    };
    let y_check_idx = if block_chunk_pos.y == 0 && vertex_offset.y < 0 {
        0
    } else if block_chunk_pos.y == max_idx && vertex_offset.y > 0 {
        2
    } else {
        1
    };
    let z_check_idx = if block_chunk_pos.z == 0 && vertex_offset.z < 0 {
        0
    } else if block_chunk_pos.z == max_idx && vertex_offset.z > 0 {
        2
    } else {
        1
    };

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

    if max_lod > center_lod {
        let grid_scale = (1 << *max_lod) as f32;
        let snap = |c: f32| (c / grid_scale).round() * grid_scale;
        [
            snap(vertex_world_pos[0]),
            snap(vertex_world_pos[1]),
            snap(vertex_world_pos[2]),
        ]
    } else {
        vertex_world_pos
    }
}

/// Create the vertices for a specified face at a particular block position
#[instrument(skip_all)]
pub fn create_face_verts(
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
                [-half_s + fx, half_s + fy, half_s + fz],
                [half_s + fx, half_s + fy, half_s + fz],
                [half_s + fx, half_s + fy, -half_s + fz],
                [-half_s + fx, half_s + fy, -half_s + fz],
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

    let face_idx = *face as usize;
    let v_offsets = &VERTEX_OFFSETS[face_idx];

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

    let final_vertices = vec![
        WorldVertex::new(
            snapped_verts[0],
            normal,
            [ao_values[0]; 3],
            uvs[0],
            tex_index,
        ),
        WorldVertex::new(
            snapped_verts[1],
            normal,
            [ao_values[1]; 3],
            uvs[1],
            tex_index,
        ),
        WorldVertex::new(
            snapped_verts[2],
            normal,
            [ao_values[2]; 3],
            uvs[2],
            tex_index,
        ),
        WorldVertex::new(
            snapped_verts[3],
            normal,
            [ao_values[3]; 3],
            uvs[3],
            tex_index,
        ),
    ];

    let indices = if (ao_values[0] + ao_values[2]) > (ao_values[1] + ao_values[3]) {
        [
            base_vertex_count,
            base_vertex_count + 1,
            base_vertex_count + 2,
            base_vertex_count + 2,
            base_vertex_count + 3,
            base_vertex_count,
        ]
    } else {
        [
            base_vertex_count,
            base_vertex_count + 1,
            base_vertex_count + 3,
            base_vertex_count + 1,
            base_vertex_count + 2,
            base_vertex_count + 3,
        ]
    };

    (final_vertices, indices)
}

/// Calculates the ambient occlusion (ao) for a chunk position
#[instrument(skip_all)]
pub fn calculate_ao_for_pos(
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

/// Builds the mesh assets given chunk vertices
#[instrument(skip_all)]
pub fn build_mesh_assets(
    name: &str,
    opaque_verts: Vec<WorldVertex>,
    opaque_idxs: Vec<u32>,
    transparent_verts: Vec<WorldVertex>,
    transparent_idxs: Vec<u32>,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    let opaque = if !opaque_verts.is_empty() {
        Some(OpaqueMeshData {
            name: name.to_string(),
            vertices: opaque_verts,
            indices: opaque_idxs,
        })
    } else {
        None
    };
    let trans = if !transparent_verts.is_empty() {
        Some(TransparentMeshData {
            name: format!("{}_trans", name),
            vertices: transparent_verts,
            indices: transparent_idxs,
        })
    } else {
        None
    };
    (opaque, trans)
}
