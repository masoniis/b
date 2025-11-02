use crate::prelude::*;
use crate::render_world::types::Vertex;
use crate::simulation_world::asset_management::MeshAsset;
use crate::simulation_world::block::block_registry::BlockId;
use crate::simulation_world::chunk::{ChunkNeighborData, Y_SHIFT, Z_SHIFT};
use crate::simulation_world::{
    asset_management::texture_map_registry::TextureMapResource,
    block::BlockRegistryResource,
    chunk::{ChunkBlocksComponent, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH},
};

type OpaqueMeshData = MeshAsset;
type TransparentMeshData = MeshAsset;

const AIR_BLOCK: BlockId = 0;

/// Doesn't really matter what block this is, but it ensures that the chunks on
/// the edge of render distance don't mesh their edge since a different block
/// is detected so it wouldn't be visible
const SOLID_VOID_BLOCK: BlockId = 1;

/// Helper function to get a block, checking neighbors if coordinates are out of bounds.
#[inline(always)]
fn get_block_with_neighbors<'a>(
    x: isize,
    y: isize,
    z: isize,
    current_chunk: &'a ChunkBlocksComponent,
    neighbors: &'a ChunkNeighborData,
) -> &'a BlockId {
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
        None => &SOLID_VOID_BLOCK,
    }
}

// A struct to hold the properties of a face, so we can compare them.
#[derive(PartialEq, Eq, Clone, Copy)]
struct FaceData {
    texture_index: u32,
    is_transparent: bool,
}

/// Helper function to build a mesh for a single chunk, considering neighbors.
#[instrument(skip_all)]
pub fn build_chunk_mesh(
    name: &str,
    chunk: &ChunkBlocksComponent,
    neighbors: &ChunkNeighborData,
    texture_map: &TextureMapResource,
    block_registry: &BlockRegistryResource,
) -> (Option<OpaqueMeshData>, Option<TransparentMeshData>) {
    let mut opaque_vertices: Vec<Vertex> = Vec::new();
    let mut opaque_indices: Vec<u32> = Vec::new();
    let mut transparent_vertices: Vec<Vertex> = Vec::new();
    let mut transparent_indices: Vec<u32> = Vec::new();

    // bitmask to track which of the 6 faces of a block have been visited already
    // 0b000001 = +X, 0b000010 = -X
    // 0b000100 = +Y, 0b001000 = -Y
    // 0b010000 = +Z, 0b100000 = -Z
    let mut visited_mask = vec![0u8; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH];
    let mask_idx = |x: usize, y: usize, z: usize| (y << Y_SHIFT) | (z << Z_SHIFT) | x;

    // sweep across each 2d plane of the 3d chunk (0=X, 1=Y, 2=Z)
    for axis in 0..3 {
        // u and v represent the 2d plane for the meshes
        //
        // d is the slices of these mesh planes
        let (d, u, v) = match axis {
            0 => (0, 1, 2), // X-axis: u=Y, v=Z
            1 => (1, 0, 2), // Y-axis: u=X, v=Z
            2 => (2, 0, 1), // Z-axis: u=X, v=Y
            _ => unreachable!(),
        };

        // chunk dimensions for these axes
        let dims = [CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH];
        let dim_d = dims[d];
        let dim_u = dims[u];
        let dim_v = dims[v];

        // loop over the 3d grid of the chunk
        for i_d in 0..dim_d {
            for i_u in 0..dim_u {
                for i_v in 0..dim_v {
                    // map (i_d, i_u, i_v) back to (x, y, z)
                    let mut pos = [0, 0, 0];
                    pos[d] = i_d;
                    pos[u] = i_u;
                    pos[v] = i_v;
                    let (x, y, z) = (pos[0], pos[1], pos[2]);

                    // mesh across both sides of the 2d plane
                    for back_face in [false, true] {
                        let face_bit = 1u8 << (axis * 2 + if back_face { 1 } else { 0 }); // 0 through 5
                        if (visited_mask[mask_idx(x, y, z)] & face_bit) != 0 {
                            continue; // face is already meshed
                        }

                        // get current block and neighbor properties to compare
                        let (ix, iy, iz) = (x as isize, y as isize, z as isize);
                        let current_block_id =
                            get_block_with_neighbors(ix, iy, iz, chunk, neighbors);

                        // if current block is air, nothing to mesh
                        if *current_block_id == AIR_BLOCK {
                            continue;
                        }

                        let mut dir = [0, 0, 0];
                        dir[d] = if back_face { -1 } else { 1 };

                        let neighbor_block_id = get_block_with_neighbors(
                            ix + dir[0],
                            iy + dir[1],
                            iz + dir[2],
                            chunk,
                            neighbors,
                        );

                        let current_props = block_registry.get(*current_block_id);
                        let neighbor_props = block_registry.get(*neighbor_block_id);

                        // culling logic
                        let should_mesh = if current_props.is_transparent {
                            if neighbor_props.is_transparent {
                                // mesh only if they are different blocks
                                *current_block_id != *neighbor_block_id
                            } else {
                                // neighbor is opaque, so we can see this face, must mesh
                                true
                            }
                        } else {
                            // only mesh if neighbor is transparent
                            neighbor_props.is_transparent
                        };

                        if !should_mesh {
                            continue;
                        }

                        // get the texture for current face
                        let tex_id = match (axis, back_face) {
                            (0, false) => &current_props.textures.right, // +X
                            (0, true) => &current_props.textures.left,   // -X
                            (1, false) => &current_props.textures.top,   // +Y
                            (1, true) => &current_props.textures.bottom, // -Y
                            (2, false) => &current_props.textures.front, // +Z
                            (2, true) => &current_props.textures.back,   // -Z
                            _ => unreachable!(),
                        };

                        let face_data = FaceData {
                            texture_index: texture_map.registry.get(*tex_id),
                            is_transparent: current_props.is_transparent,
                        };

                        // INFO: -------------------------------
                        //         greedy mesh expansion
                        // -------------------------------------

                        // find mesh tile width
                        let mut width = 1;
                        while i_u + width < dim_u {
                            let mut pos_w = [0, 0, 0];
                            pos_w[d] = i_d;
                            pos_w[u] = i_u + width;
                            pos_w[v] = i_v;
                            let (wx, wy, wz) = (pos_w[0], pos_w[1], pos_w[2]);

                            // if face is already visited, expansion is broken
                            if (visited_mask[mask_idx(wx, wy, wz)] & face_bit) != 0 {
                                break;
                            }

                            // culling and face property analysis
                            let w_current_id = get_block_with_neighbors(
                                wx as isize,
                                wy as isize,
                                wz as isize,
                                chunk,
                                neighbors,
                            );
                            let w_neighbor_id = get_block_with_neighbors(
                                wx as isize + dir[0],
                                wy as isize + dir[1],
                                wz as isize + dir[2],
                                chunk,
                                neighbors,
                            );
                            let w_current_props = block_registry.get(*w_current_id);
                            let w_neighbor_props = block_registry.get(*w_neighbor_id);

                            if w_current_props.is_transparent != w_neighbor_props.is_transparent
                                && *w_current_id != AIR_BLOCK
                            {
                                let w_tex_id = match (axis, back_face) {
                                    (0, false) => &w_current_props.textures.right,
                                    (0, true) => &w_current_props.textures.left,
                                    (1, false) => &w_current_props.textures.top,
                                    (1, true) => &w_current_props.textures.bottom,
                                    (2, false) => &w_current_props.textures.front,
                                    (2, true) => &w_current_props.textures.back,
                                    _ => unreachable!(),
                                };
                                let w_face_data = FaceData {
                                    texture_index: texture_map.registry.get(*w_tex_id),
                                    is_transparent: w_current_props.is_transparent,
                                };
                                if w_face_data == face_data {
                                    width += 1;
                                } else {
                                    break; // different face type
                                }
                            } else {
                                break; // culling failed, stop expansion
                            }
                        }

                        // find mesh tile height
                        let mut height = 1;
                        'height_loop: while i_v + height < dim_v {
                            // check every block in current width row,
                            // they must all "pass" to expand height
                            for w in 0..width {
                                let mut pos_h = [0, 0, 0];
                                pos_h[d] = i_d;
                                pos_h[u] = i_u + w;
                                pos_h[v] = i_v + height;
                                let (hx, hy, hz) = (pos_h[0], pos_h[1], pos_h[2]);

                                if (visited_mask[mask_idx(hx, hy, hz)] & face_bit) != 0 {
                                    break 'height_loop;
                                }

                                let h_current_id = get_block_with_neighbors(
                                    hx as isize,
                                    hy as isize,
                                    hz as isize,
                                    chunk,
                                    neighbors,
                                );
                                let h_neighbor_id = get_block_with_neighbors(
                                    hx as isize + dir[0],
                                    hy as isize + dir[1],
                                    hz as isize + dir[2],
                                    chunk,
                                    neighbors,
                                );
                                let h_current_props = block_registry.get(*h_current_id);
                                let h_neighbor_props = block_registry.get(*h_neighbor_id);

                                if h_current_props.is_transparent != h_neighbor_props.is_transparent
                                    && *h_current_id != AIR_BLOCK
                                {
                                    let h_tex_id = match (axis, back_face) {
                                        (0, false) => &h_current_props.textures.right,
                                        (0, true) => &h_current_props.textures.left,
                                        (1, false) => &h_current_props.textures.top,
                                        (1, true) => &h_current_props.textures.bottom,
                                        (2, false) => &h_current_props.textures.front,
                                        (2, true) => &h_current_props.textures.back,
                                        _ => unreachable!(),
                                    };
                                    let h_face_data = FaceData {
                                        texture_index: texture_map.registry.get(*h_tex_id),
                                        is_transparent: h_current_props.is_transparent,
                                    };
                                    if h_face_data != face_data {
                                        break 'height_loop; // different face type
                                    }
                                } else {
                                    break 'height_loop; // culling failed, stop expansion
                                }
                            }

                            // entire row passed, increase height
                            height += 1;
                        }

                        // INFO: --------------------------------------------------
                        //         quad construction and visitation marking
                        // --------------------------------------------------------

                        let (target_vertices, target_indices) = if face_data.is_transparent {
                            (&mut transparent_vertices, &mut transparent_indices)
                        } else {
                            (&mut opaque_vertices, &mut opaque_indices)
                        };
                        let base_vertex_count = target_vertices.len() as u32;

                        let face = match (axis, back_face) {
                            (0, false) => Face::Right,
                            (0, true) => Face::Left,
                            (1, false) => Face::Top,
                            (1, true) => Face::Bottom,
                            (2, false) => Face::Front,
                            (2, true) => Face::Back,
                            _ => unreachable!(),
                        };

                        let (face_verts, face_indices) = create_quad(
                            face,
                            x,
                            y,
                            z,
                            width,
                            height,
                            face_data.texture_index,
                            base_vertex_count,
                        );
                        target_vertices.extend(face_verts);
                        target_indices.extend(face_indices);

                        // mark all quad faces as visited
                        for h in 0..height {
                            for w in 0..width {
                                let mut pos_v = [0, 0, 0];
                                pos_v[d] = i_d;
                                pos_v[u] = i_u + w;
                                pos_v[v] = i_v + h;
                                visited_mask[mask_idx(pos_v[0], pos_v[1], pos_v[2])] |= face_bit;
                            }
                        }
                    }
                }
            }
        }
    }

    // INFO: ----------------------------
    //         return mesh assets
    // ----------------------------------

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

enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

/// A new quad generator that creates a quad of a given width and height
fn create_quad(
    face: Face,
    x: usize, // starting block x,y,z
    y: usize,
    z: usize,
    width: usize,  // width of quad (in blocks)
    height: usize, // height of quad (in blocks)
    tex_index: u32,
    base_vertex_count: u32,
) -> (Vec<Vertex>, [u32; 6]) {
    let (x, y, z) = (x as f32, y as f32, z as f32);
    let (width, height) = (width as f32, height as f32);

    // define vertices and normals based on face
    let (verts, normal, uvs): (Vec<[f32; 3]>, [f32; 3], [[f32; 2]; 4]) = match face {
        Face::Top => (
            vec![
                [-0.5 + x, 0.5 + y, -0.5 + z],                  // 0 (Start)
                [-0.5 + x + width, 0.5 + y, -0.5 + z],          // 1 (End Width)
                [-0.5 + x + width, 0.5 + y, -0.5 + z + height], // 2 (End Width & Height)
                [-0.5 + x, 0.5 + y, -0.5 + z + height],         // 3 (End Height)
            ],
            [0.0, 1.0, 0.0],
            [[0.0, height], [width, height], [width, 0.0], [0.0, 0.0]],
        ),
        Face::Bottom => (
            vec![
                [-0.5 + x, -0.5 + y, -0.5 + z + height],         // 0
                [-0.5 + x + width, -0.5 + y, -0.5 + z + height], // 1
                [-0.5 + x + width, -0.5 + y, -0.5 + z],          // 2
                [-0.5 + x, -0.5 + y, -0.5 + z],                  // 3
            ],
            [0.0, -1.0, 0.0],
            [[0.0, 0.0], [width, 0.0], [width, height], [0.0, height]],
        ),
        Face::Right => (
            vec![
                [0.5 + x, -0.5 + y, -0.5 + z],                  // 0
                [0.5 + x, -0.5 + y, -0.5 + z + height],         // 1
                [0.5 + x, -0.5 + y + width, -0.5 + z + height], // 2
                [0.5 + x, -0.5 + y + width, -0.5 + z],          // 3
            ],
            [1.0, 0.0, 0.0],
            [[0.0, 0.0], [height, 0.0], [height, width], [0.0, width]],
        ),
        Face::Left => (
            vec![
                [-0.5 + x, -0.5 + y, -0.5 + z + height],         // 0
                [-0.5 + x, -0.5 + y, -0.5 + z],                  // 1
                [-0.5 + x, -0.5 + y + width, -0.5 + z],          // 2
                [-0.5 + x, -0.5 + y + width, -0.5 + z + height], // 3
            ],
            [-1.0, 0.0, 0.0],
            [[0.0, 0.0], [height, 0.0], [height, width], [0.0, width]],
        ),
        Face::Front => (
            vec![
                [-0.5 + x + width, -0.5 + y, 0.5 + z],          // 0
                [-0.5 + x, -0.5 + y, 0.5 + z],                  // 1
                [-0.5 + x, -0.5 + y + height, 0.5 + z],         // 2
                [-0.5 + x + width, -0.5 + y + height, 0.5 + z], // 3
            ],
            [0.0, 0.0, 1.0],
            [[width, 0.0], [0.0, 0.0], [0.0, height], [width, height]],
        ),
        Face::Back => (
            vec![
                [-0.5 + x, -0.5 + y, -0.5 + z],                  // 0
                [-0.5 + x + width, -0.5 + y, -0.5 + z],          // 1
                [-0.5 + x + width, -0.5 + y + height, -0.5 + z], // 2
                [-0.5 + x, -0.5 + y + height, -0.5 + z],         // 3
            ],
            [0.0, 0.0, -1.0],
            [[0.0, 0.0], [width, 0.0], [width, height], [0.0, height]],
        ),
    };

    // create the 4 vertices for this quad
    let final_vertices = vec![
        Vertex::new(verts[0], normal, uvs[0], tex_index),
        Vertex::new(verts[1], normal, uvs[1], tex_index),
        Vertex::new(verts[2], normal, uvs[2], tex_index),
        Vertex::new(verts[3], normal, uvs[3], tex_index),
    ];

    // indices in clockwise order
    let indices = [
        base_vertex_count + 0,
        base_vertex_count + 2,
        base_vertex_count + 1,
        base_vertex_count + 0,
        base_vertex_count + 3,
        base_vertex_count + 2,
    ];

    (final_vertices, indices)
}
