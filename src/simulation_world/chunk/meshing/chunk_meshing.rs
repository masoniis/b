use crate::prelude::*;
use crate::render_world::types::Vertex;
use crate::simulation_world::chunk::async_chunking::ChunkNeighborData;
use crate::simulation_world::{
    asset_management::texture_map_registry::TextureMapResource,
    block::{property_registry::BlockRegistryResource, Block},
    chunk::{ChunkBlocksComponent, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH},
};

const AIR_BLOCK: Block = Block { id: 0 };

/// doesn't really matter what block this is, but it ensures that the chunks on
/// the edge of render distance don't mesh their edge since a different block
/// is detected
const SOLID_VOID_BLOCK: Block = Block { id: 1 };

/// Helper function to get a block, checking neighbors if coordinates are out of bounds.
#[inline(always)]
fn get_block_with_neighbors<'a>(
    x: isize,
    y: isize,
    z: isize,
    current_chunk: &'a ChunkBlocksComponent,
    neighbors: &'a ChunkNeighborData,
) -> &'a Block {
    // check if we are in bounds
    if x >= 0
        && x < CHUNK_WIDTH as isize
        && y >= 0
        && y < CHUNK_HEIGHT as isize
        && z >= 0
        && z < CHUNK_DEPTH as isize
    {
        return current_chunk
            .get_block(x as usize, y as usize, z as usize)
            .unwrap_or(&AIR_BLOCK);
    }

    // if out, determine the neighbor to check
    let (neighbor_chunk, nx, ny, nz) = if x < 0 {
        (&neighbors.left, x + CHUNK_WIDTH as isize, y, z)
    } else if x >= CHUNK_WIDTH as isize {
        (&neighbors.right, x - CHUNK_WIDTH as isize, y, z)
    } else if y < 0 {
        (&neighbors.bottom, x, y + CHUNK_HEIGHT as isize, z)
    } else if y >= CHUNK_HEIGHT as isize {
        (&neighbors.top, x, y - CHUNK_HEIGHT as isize, z)
    } else if z < 0 {
        (&neighbors.back, x, y, z + CHUNK_DEPTH as isize)
    } else if z >= CHUNK_DEPTH as isize {
        (&neighbors.front, x, y, z - CHUNK_DEPTH as isize)
    } else {
        error!("Logic error in get_block_with_neighbors");
        return &AIR_BLOCK;
    };

    // and then check the neighbor
    match neighbor_chunk {
        Some(neighbor_data) => neighbor_data
            .get_block(nx as usize, ny as usize, nz as usize)
            .unwrap_or(&AIR_BLOCK),
        None => {
            &SOLID_VOID_BLOCK // neighbor is out of bounds, assume air
        }
    }
}

/// Helper function to build a mesh for a single chunk, considering neighbors.
pub fn build_chunk_mesh(
    chunk: &ChunkBlocksComponent,
    neighbors: &ChunkNeighborData,
    texture_map: &TextureMapResource,
    block_registry: &BlockRegistryResource,
) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_DEPTH {
            for x in 0..CHUNK_WIDTH {
                // cast to isize for neighbor checking
                let (ix, iy, iz) = (x as isize, y as isize, z as isize);

                // skip air blocks
                let block = chunk.get_block(x, y, z).unwrap_or(&AIR_BLOCK);
                if block.id == AIR_BLOCK.id {
                    continue;
                }

                let block_properties = block_registry.get(block.id);

                // +Y (Top)
                let neighbor_top = get_block_with_neighbors(ix, iy + 1, iz, chunk, neighbors);
                if block_registry.get(neighbor_top.id).is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.top;
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Top, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // -Y (Bottom)
                let neighbor_bottom = get_block_with_neighbors(ix, iy - 1, iz, chunk, neighbors);
                if block_registry.get(neighbor_bottom.id).is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.bottom;
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Bottom, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // -X (Left / West)
                let neighbor_left = get_block_with_neighbors(ix - 1, iy, iz, chunk, neighbors);
                if block_registry.get(neighbor_left.id).is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.west;
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Left, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // +X (Right / East)
                let neighbor_right = get_block_with_neighbors(ix + 1, iy, iz, chunk, neighbors);
                if block_registry.get(neighbor_right.id).is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.east;
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Right, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // +Z (Front / South)
                let neighbor_front = get_block_with_neighbors(ix, iy, iz + 1, chunk, neighbors);
                if block_registry.get(neighbor_front.id).is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.south;
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Front, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // -Z (Back / North)
                let neighbor_back = get_block_with_neighbors(ix, iy, iz - 1, chunk, neighbors);
                if block_registry.get(neighbor_back.id).is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.north;
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Back, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }
            }
        }
    }
    (vertices, indices)
}

enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

fn get_face(
    face: Face,
    x: usize,
    y: usize,
    z: usize,
    tex_index: u32,
    base_vertex_count: u32,
) -> (Vec<Vertex>, [u32; 6]) {
    let (fx, fy, fz) = (x as f32, y as f32, z as f32);

    let (verts, normal): (Vec<[f32; 3]>, [f32; 3]) = match face {
        Face::Top => (
            vec![
                [-0.5 + fx, 0.5 + fy, 0.5 + fz],
                [0.5 + fx, 0.5 + fy, 0.5 + fz],
                [0.5 + fx, 0.5 + fy, -0.5 + fz],
                [-0.5 + fx, 0.5 + fy, -0.5 + fz],
            ],
            [0.0, 1.0, 0.0],
        ), // +Y Normal
        Face::Bottom => (
            vec![
                [-0.5 + fx, -0.5 + fy, -0.5 + fz],
                [0.5 + fx, -0.5 + fy, -0.5 + fz],
                [0.5 + fx, -0.5 + fy, 0.5 + fz],
                [-0.5 + fx, -0.5 + fy, 0.5 + fz],
            ],
            [0.0, -1.0, 0.0],
        ), // -Y Normal
        Face::Left => (
            vec![
                // -X Face
                [-0.5 + fx, -0.5 + fy, -0.5 + fz],
                [-0.5 + fx, -0.5 + fy, 0.5 + fz],
                [-0.5 + fx, 0.5 + fy, 0.5 + fz],
                [-0.5 + fx, 0.5 + fy, -0.5 + fz],
            ],
            [-1.0, 0.0, 0.0],
        ), // -X Normal
        Face::Right => (
            vec![
                // +X Face
                [0.5 + fx, -0.5 + fy, 0.5 + fz],
                [0.5 + fx, -0.5 + fy, -0.5 + fz],
                [0.5 + fx, 0.5 + fy, -0.5 + fz],
                [0.5 + fx, 0.5 + fy, 0.5 + fz],
            ],
            [1.0, 0.0, 0.0],
        ), // +X Normal
        Face::Front => (
            vec![
                // +Z Face
                [-0.5 + fx, -0.5 + fy, 0.5 + fz],
                [0.5 + fx, -0.5 + fy, 0.5 + fz],
                [0.5 + fx, 0.5 + fy, 0.5 + fz],
                [-0.5 + fx, 0.5 + fy, 0.5 + fz],
            ],
            [0.0, 0.0, 1.0],
        ), // +Z Normal
        Face::Back => (
            vec![
                // -Z Face
                [0.5 + fx, -0.5 + fy, -0.5 + fz],
                [-0.5 + fx, -0.5 + fy, -0.5 + fz],
                [-0.5 + fx, 0.5 + fy, -0.5 + fz],
                [0.5 + fx, 0.5 + fy, -0.5 + fz],
            ],
            [0.0, 0.0, -1.0],
        ), // -Z Normal
    };

    // Define standard UVs
    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
    let final_vertices = vec![
        Vertex::new(verts[0], normal, uvs[0], tex_index),
        Vertex::new(verts[1], normal, uvs[1], tex_index),
        Vertex::new(verts[2], normal, uvs[2], tex_index),
        Vertex::new(verts[3], normal, uvs[3], tex_index),
    ];

    let indices = [
        base_vertex_count + 0,
        base_vertex_count + 1,
        base_vertex_count + 2,
        base_vertex_count + 2,
        base_vertex_count + 3,
        base_vertex_count + 0,
    ];

    (final_vertices, indices)
}
