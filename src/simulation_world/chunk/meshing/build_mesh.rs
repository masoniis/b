use crate::prelude::*;
use crate::render_world::types::{TextureId, Vertex};
use crate::simulation_world::asset_management::MeshAsset;
use crate::simulation_world::block::block_registry::{BlockId, AIR_BLOCK_ID};
use crate::simulation_world::block::BlockProperties;
use crate::simulation_world::chunk::padded_chunk_view::PaddedChunkView;
use crate::simulation_world::{
    asset_management::texture_map_registry::TextureMapResource,
    block::BlockRegistryResource,
    chunk::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH},
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
        get_texture: |props| props.textures.back,
    },
    FaceDirection {
        offset: IVec3::new(0, 0, -1),
        face: Face::Back,
        get_texture: |props| props.textures.front,
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

/// Build a mesh for a single chunk, considering neighbors.
#[instrument(skip_all)]
pub fn build_chunk_mesh(
    name: &str,
    padded_chunk: PaddedChunkView,
    texture_map: &TextureMapResource,
    block_registry: &BlockRegistryResource,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    let mut opaque_vertices: Vec<Vertex> = Vec::new();
    let mut opaque_indices: Vec<u32> = Vec::new();
    let mut transparent_vertices: Vec<Vertex> = Vec::new();
    let mut transparent_indices: Vec<u32> = Vec::new();

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_DEPTH {
            for x in 0..CHUNK_WIDTH {
                let pos = IVec3::new(x as i32, y as i32, z as i32);

                let current_block_id = padded_chunk.get_block(pos);

                // skip air blocks
                if current_block_id == AIR_BLOCK_ID {
                    continue;
                }

                let current_block_props = block_registry.get(current_block_id);

                let (target_vertices, target_indices) = if current_block_props.is_transparent {
                    (&mut transparent_vertices, &mut transparent_indices)
                } else {
                    (&mut opaque_vertices, &mut opaque_indices)
                };

                // check all 6 faces
                for direction in &FACE_DIRECTIONS {
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

                        // AO calculation for the face
                        let face_index = direction.face as usize;
                        let ao_values = [
                            AO_MAPPING[get_ao(
                                pos,
                                AO_OFFSETS[face_index][0][0],
                                AO_OFFSETS[face_index][0][1],
                                AO_OFFSETS[face_index][0][2],
                                &padded_chunk,
                                block_registry,
                            ) as usize],
                            AO_MAPPING[get_ao(
                                pos,
                                AO_OFFSETS[face_index][1][0],
                                AO_OFFSETS[face_index][1][1],
                                AO_OFFSETS[face_index][1][2],
                                &padded_chunk,
                                block_registry,
                            ) as usize],
                            AO_MAPPING[get_ao(
                                pos,
                                AO_OFFSETS[face_index][2][0],
                                AO_OFFSETS[face_index][2][1],
                                AO_OFFSETS[face_index][2][2],
                                &padded_chunk,
                                block_registry,
                            ) as usize],
                            AO_MAPPING[get_ao(
                                pos,
                                AO_OFFSETS[face_index][3][0],
                                AO_OFFSETS[face_index][3][1],
                                AO_OFFSETS[face_index][3][2],
                                &padded_chunk,
                                block_registry,
                            ) as usize],
                        ];

                        let (face_verts, face_indices) = create_face_verts(
                            &direction.face,
                            x,
                            y,
                            z,
                            tex_index,
                            base_vertex_count,
                            ao_values,
                        );
                        target_vertices.extend(face_verts);
                        target_indices.extend(face_indices);
                    }
                }
            }
        }
    }

    let opaque_mesh = if !opaque_vertices.is_empty() {
        Some(OpaqueMeshData {
            name: name.to_string(),
            vertices: opaque_vertices,
            indices: opaque_indices,
        })
    } else {
        None
    };

    let transparent_mesh = if !transparent_vertices.is_empty() {
        Some(TransparentMeshData {
            name: format!("{}_transparent", name),
            vertices: transparent_vertices,
            indices: transparent_indices,
        })
    } else {
        None
    };

    (opaque_mesh, transparent_mesh)
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

fn create_face_verts(
    face: &Face,
    x: usize,
    y: usize,
    z: usize,
    tex_index: u32,
    base_vertex_count: u32,
    ao_values: [f32; 4], // AO values for v0, v1, v2, v3
) -> (Vec<Vertex>, [u32; 6]) {
    let (fx, fy, fz) = (x as f32, y as f32, z as f32);

    let (verts, normal): (Vec<[f32; 3]>, [f32; 3]) = match face {
        Face::Top => (
            vec![
                [-0.5 + fx, 0.5 + fy, 0.5 + fz],  // v0
                [0.5 + fx, 0.5 + fy, 0.5 + fz],   // v1
                [0.5 + fx, 0.5 + fy, -0.5 + fz],  // v2
                [-0.5 + fx, 0.5 + fy, -0.5 + fz], // v3
            ],
            [0.0, 1.0, 0.0],
        ),
        Face::Bottom => (
            vec![
                [-0.5 + fx, -0.5 + fy, -0.5 + fz],
                [0.5 + fx, -0.5 + fy, -0.5 + fz],
                [0.5 + fx, -0.5 + fy, 0.5 + fz],
                [-0.5 + fx, -0.5 + fy, 0.5 + fz],
            ],
            [0.0, -1.0, 0.0],
        ),
        Face::Left => (
            vec![
                [-0.5 + fx, -0.5 + fy, -0.5 + fz],
                [-0.5 + fx, -0.5 + fy, 0.5 + fz],
                [-0.5 + fx, 0.5 + fy, 0.5 + fz],
                [-0.5 + fx, 0.5 + fy, -0.5 + fz],
            ],
            [-1.0, 0.0, 0.0],
        ),
        Face::Right => (
            vec![
                [0.5 + fx, -0.5 + fy, 0.5 + fz],
                [0.5 + fx, -0.5 + fy, -0.5 + fz],
                [0.5 + fx, 0.5 + fy, -0.5 + fz],
                [0.5 + fx, 0.5 + fy, 0.5 + fz],
            ],
            [1.0, 0.0, 0.0],
        ),
        Face::Front => (
            vec![
                [-0.5 + fx, -0.5 + fy, 0.5 + fz],
                [0.5 + fx, -0.5 + fy, 0.5 + fz],
                [0.5 + fx, 0.5 + fy, 0.5 + fz],
                [-0.5 + fx, 0.5 + fy, 0.5 + fz],
            ],
            [0.0, 0.0, 1.0],
        ),
        Face::Back => (
            vec![
                [0.5 + fx, -0.5 + fy, -0.5 + fz],
                [-0.5 + fx, -0.5 + fy, -0.5 + fz],
                [-0.5 + fx, 0.5 + fy, -0.5 + fz],
                [0.5 + fx, 0.5 + fy, -0.5 + fz],
            ],
            [0.0, 0.0, -1.0],
        ),
    };

    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    // apply AO to color for the 4 vertices
    let final_vertices = vec![
        Vertex::new(
            verts[0],
            normal,
            [ao_values[0], ao_values[0], ao_values[0]],
            uvs[0],
            tex_index,
        ),
        Vertex::new(
            verts[1],
            normal,
            [ao_values[1], ao_values[1], ao_values[1]],
            uvs[1],
            tex_index,
        ),
        Vertex::new(
            verts[2],
            normal,
            [ao_values[2], ao_values[2], ao_values[2]],
            uvs[2],
            tex_index,
        ),
        Vertex::new(
            verts[3],
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
