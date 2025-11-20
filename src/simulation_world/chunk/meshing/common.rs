use super::{OpaqueMeshData, TransparentMeshData};
use crate::prelude::*;
use crate::render_world::textures::TextureRegistryResource;
use crate::render_world::types::{TextureId, WorldVertex};
use crate::simulation_world::{
    block::{BlockId, BlockRegistryResource},
    chunk::{types::ChunkLod, NeighborLODs, PaddedChunk},
};

// INFO: -----------------------
//         lookup tables
// -----------------------------

/// The 6 cardinal neighbor offsets.
///
/// Order: Top, Bottom, Left, Right, Front, Back
pub const NEIGHBOR_OFFSETS: [IVec3; 6] = [
    IVec3::new(0, 1, 0),  // Top
    IVec3::new(0, -1, 0), // Bottom
    IVec3::new(-1, 0, 0), // Left
    IVec3::new(1, 0, 0),  // Right
    IVec3::new(0, 0, 1),  // Front
    IVec3::new(0, 0, -1), // Back
];

/// Normals for the 6 faces.
///
/// Order: Top, Bottom, Left, Right, Front, Back
pub const FACE_NORMALS: [[f32; 3]; 6] = [
    [0.0, 1.0, 0.0],  // Top
    [0.0, -1.0, 0.0], // Bottom
    [-1.0, 0.0, 0.0], // Left
    [1.0, 0.0, 0.0],  // Right
    [0.0, 0.0, 1.0],  // Front
    [0.0, 0.0, -1.0], // Back
];

/// Raw unit-cube vertex positions (0.0 to 1.0) for all 6 faces * 4 verts.
///
/// Order: Top, Bottom, Left, Right, Front, Back
pub const UNIT_VERTICES: [[[f32; 3]; 4]; 6] = [
    // Top (Y+)
    [
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ],
    // Bottom (Y-)
    [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ],
    // Left (X-)
    [
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [0.0, 1.0, 0.0],
    ],
    // Right (X+)
    [
        [1.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
    ],
    // Front (Z+)
    [
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ],
    // Back (Z-)
    [
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
    ],
];

/// Used for LOD stitching checks.
pub const VERTEX_OFFSETS: [[IVec3; 4]; 6] = [
    [
        IVec3::new(-1, 1, 1),
        IVec3::new(1, 1, 1),
        IVec3::new(1, 1, -1),
        IVec3::new(-1, 1, -1),
    ], // Top
    [
        IVec3::new(-1, -1, -1),
        IVec3::new(1, -1, -1),
        IVec3::new(1, -1, 1),
        IVec3::new(-1, -1, 1),
    ], // Bottom
    [
        IVec3::new(-1, -1, -1),
        IVec3::new(-1, -1, 1),
        IVec3::new(-1, 1, 1),
        IVec3::new(-1, 1, -1),
    ], // Left
    [
        IVec3::new(1, -1, 1),
        IVec3::new(1, -1, -1),
        IVec3::new(1, 1, -1),
        IVec3::new(1, 1, 1),
    ], // Right
    [
        IVec3::new(-1, -1, 1),
        IVec3::new(1, -1, 1),
        IVec3::new(1, 1, 1),
        IVec3::new(-1, 1, 1),
    ], // Front
    [
        IVec3::new(1, -1, -1),
        IVec3::new(-1, -1, -1),
        IVec3::new(-1, 1, -1),
        IVec3::new(1, 1, -1),
    ], // Back
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

// INFO: ------------------------
//         util functions
// ------------------------------

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
    padded_chunk: &PaddedChunk,
    block_registry: &BlockRegistryResource,
) -> u8 {
    let s1_pos = pos + side1_off;
    let s1 = !block_registry
        .get(padded_chunk.get_block(s1_pos.x, s1_pos.y, s1_pos.z))
        .is_transparent;
    let s2_pos = pos + side2_off;
    let s2 = !block_registry
        .get(padded_chunk.get_block(s2_pos.x, s2_pos.y, s2_pos.z))
        .is_transparent;
    let c_pos = pos + corner_off;
    let c = !block_registry
        .get(padded_chunk.get_block(c_pos.x, c_pos.y, c_pos.z))
        .is_transparent;

    if s1 && s2 {
        3
    } else {
        (s1 as u8) + (s2 as u8) + (c as u8)
    }
}

/// Calculates the snapped position of a vertex based on the max LOD of all chunks sharing it.
pub fn get_snapped_pos(
    vertex_world_pos: [f32; 3],
    vertex_offset: IVec3,
    block_chunk_pos: IVec3,
    chunk_size: usize,
    center_lod: ChunkLod,
    all_lods: &NeighborLODs,
) -> [f32; 3] {
    let max_idx = (chunk_size as i32) - 1;
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

/// Calculates the ambient occlusion (ao) for a chunk position
pub fn calculate_ao_for_pos(
    pos: IVec3,
    face_idx: usize,
    padded_chunk: &PaddedChunk,
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

pub struct MesherContext<'a> {
    pub padded_chunk: &'a PaddedChunk,
    pub block_registry: &'a BlockRegistryResource,
    pub texture_map: &'a TextureRegistryResource,
    pub center_lod: ChunkLod,
    pub neighbor_lods: &'a NeighborLODs,
    pub chunk_size: usize,
    pub scale: f32,
}

impl<'a> MesherContext<'a> {
    #[inline(always)]
    pub fn push_face(
        &self,
        face_idx: usize,
        block_pos: IVec3,
        tex_id: TextureId,
        ao_values: [f32; 4],
        out_vertices: &mut Vec<WorldVertex>,
        out_indices: &mut Vec<u32>,
    ) {
        let tex_index = self.texture_map.get(tex_id);
        let base_idx = out_vertices.len() as u32;
        let normal = FACE_NORMALS[face_idx];
        let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

        let unit_verts = &UNIT_VERTICES[face_idx];
        let lod_offsets = &VERTEX_OFFSETS[face_idx];

        for i in 0..4 {
            let x = (unit_verts[i][0] + block_pos.x as f32) * self.scale;
            let y = (unit_verts[i][1] + block_pos.y as f32) * self.scale;
            let z = (unit_verts[i][2] + block_pos.z as f32) * self.scale;

            let snapped = get_snapped_pos(
                [x, y, z],
                lod_offsets[i],
                block_pos,
                self.chunk_size,
                self.center_lod,
                self.neighbor_lods,
            );

            out_vertices.push(WorldVertex::new(
                snapped,
                normal,
                [ao_values[i]; 3],
                uvs[i],
                tex_index,
            ));
        }

        if (ao_values[0] + ao_values[2]) > (ao_values[1] + ao_values[3]) {
            out_indices.extend_from_slice(&[
                base_idx,
                base_idx + 1,
                base_idx + 2,
                base_idx + 2,
                base_idx + 3,
                base_idx,
            ]);
        } else {
            out_indices.extend_from_slice(&[
                base_idx,
                base_idx + 1,
                base_idx + 3,
                base_idx + 1,
                base_idx + 2,
                base_idx + 3,
            ]);
        }
    }
}
