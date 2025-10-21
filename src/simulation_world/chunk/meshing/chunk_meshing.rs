use crate::prelude::*;
use crate::render_world::types::Vertex;
use crate::simulation_world::asset_management::{AssetStorageResource, MeshAsset};
use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::block::Block;
use crate::simulation_world::chunk::chunk::Chunk;
use crate::simulation_world::chunk::{MeshComponent, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::simulation_world::global_resources::texture_map::TextureMapResource;
use bevy_ecs::prelude::*;

/// Finds all new chunks and generates a mesh for them.
#[instrument(skip_all)]
pub fn chunk_meshing_system(
    // Input
    new_chunk_query: Query<(Entity, &Chunk), (Added<Chunk>, Without<MeshComponent>)>,
    texture_map: Res<TextureMapResource>,
    block_registry: Res<BlockRegistryResource>,

    // Output (inserted chunk meshes)
    mesh_assets: Res<AssetStorageResource<MeshAsset>>,
    mut commands: Commands,
) {
    for (entity, chunk) in new_chunk_query.iter() {
        let (vertices, indices) = build_chunk_mesh(chunk, &texture_map, &block_registry);

        if !vertices.is_empty() {
            info!(
                "Generated mesh for chunk ({}, {}, {}) with {} vertices and {} indices",
                chunk.x,
                chunk.y,
                chunk.z,
                vertices.len(),
                indices.len()
            );

            let mesh_asset = MeshAsset {
                name: format!("chunk_{}_{}_{}", chunk.x, chunk.y, chunk.z),
                vertices,
                indices,
            };

            let mesh_handle = mesh_assets.add(mesh_asset);

            commands
                .entity(entity)
                .insert(MeshComponent::new(mesh_handle));
        }
    }
}

/// Helper function to build a mesh for a single chunk
fn build_chunk_mesh(
    chunk: &Chunk,
    texture_map: &TextureMapResource,
    block_registry: &BlockRegistryResource,
) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let air_block = Block { id: 0 };

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_DEPTH {
            for x in 0..CHUNK_WIDTH {
                let block = chunk.get_block(x, y, z).unwrap_or(&air_block);
                if block.id == 0 {
                    continue; // no need to mesh air
                }
                let block_properties = block_registry.get(block.id);

                // --- Check neighbor +Y (Top Face) ---
                let neighbor_top = if y < CHUNK_HEIGHT - 1 {
                    chunk.get_block(x, y + 1, z).unwrap_or(&air_block)
                } else {
                    &air_block
                };
                let neighbor_top_props = block_registry.get(neighbor_top.id);
                if neighbor_top_props.is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.top;
                    let tex_index = texture_map.registry.get(*tex_id);

                    let (face_verts, face_indices) =
                        get_face(Face::Top, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // --- Check neighbor -Y (Bottom Face) ---
                let neighbor_bottom = if y > 0 {
                    chunk.get_block(x, y - 1, z).unwrap_or(&air_block)
                } else {
                    &air_block
                };
                let neighbor_bottom_props = block_registry.get(neighbor_bottom.id);
                if neighbor_bottom_props.is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.bottom;
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Bottom, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // --- Check neighbor -X (Left / West Face) ---
                let neighbor_left = if x > 0 {
                    chunk.get_block(x - 1, y, z).unwrap_or(&air_block)
                } else {
                    &air_block
                };
                let neighbor_left_props = block_registry.get(neighbor_left.id);
                if neighbor_left_props.is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.west; //
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Left, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // --- Check neighbor +X (Right / East Face) ---
                let neighbor_right = if x < CHUNK_WIDTH - 1 {
                    chunk.get_block(x + 1, y, z).unwrap_or(&air_block)
                } else {
                    &air_block
                };
                let neighbor_right_props = block_registry.get(neighbor_right.id);

                if neighbor_right_props.is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.east; //
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Right, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // --- Check neighbor +Z (Front / South Face) ---
                let neighbor_front = if z < CHUNK_DEPTH - 1 {
                    chunk.get_block(x, y, z + 1).unwrap_or(&air_block)
                } else {
                    &air_block
                };
                let neighbor_front_props = block_registry.get(neighbor_front.id);
                if neighbor_front_props.is_transparent {
                    let base_vertex_count = vertices.len() as u32;
                    let tex_id = &block_properties.textures.south;
                    let tex_index = texture_map.registry.get(*tex_id);
                    let (face_verts, face_indices) =
                        get_face(Face::Front, x, y, z, tex_index, base_vertex_count);
                    vertices.extend(face_verts);
                    indices.extend(face_indices);
                }

                // --- Check neighbor -Z (Back / North Face) ---
                let neighbor_back = if z > 0 {
                    chunk.get_block(x, y, z - 1).unwrap_or(&air_block)
                } else {
                    &air_block
                };
                let neighbor_back_props = block_registry.get(neighbor_back.id);

                if neighbor_back_props.is_transparent {
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

/// Returns the 4 vertices and 6 indices for a single cube face.
/// Note: The x, y, z are the *local block coordinates*.
fn get_face(
    face: Face,
    x: usize,
    y: usize,
    z: usize,
    tex_index: u32,
    base_vertex_count: u32,
) -> (Vec<Vertex>, [u32; 6]) {
    // The (x,y,z) are block coords, so add them to the vertex offsets
    let (fx, fy, fz) = (x as f32, y as f32, z as f32);

    let verts = match face {
        Face::Top => vec![
            Vertex::new([-0.5 + fx, 0.5 + fy, 0.5 + fz], [0.0, 0.0], tex_index),
            Vertex::new([0.5 + fx, 0.5 + fy, 0.5 + fz], [1.0, 0.0], tex_index),
            Vertex::new([0.5 + fx, 0.5 + fy, -0.5 + fz], [1.0, 1.0], tex_index),
            Vertex::new([-0.5 + fx, 0.5 + fy, -0.5 + fz], [0.0, 1.0], tex_index),
        ],
        Face::Bottom => vec![
            Vertex::new([-0.5 + fx, -0.5 + fy, -0.5 + fz], [0.0, 0.0], tex_index),
            Vertex::new([0.5 + fx, -0.5 + fy, -0.5 + fz], [1.0, 0.0], tex_index),
            Vertex::new([0.5 + fx, -0.5 + fy, 0.5 + fz], [1.0, 1.0], tex_index),
            Vertex::new([-0.5 + fx, -0.5 + fy, 0.5 + fz], [0.0, 1.0], tex_index),
        ],
        Face::Left => vec![
            Vertex::new([-0.5 + fx, -0.5 + fy, -0.5 + fz], [0.0, 0.0], tex_index),
            Vertex::new([-0.5 + fx, -0.5 + fy, 0.5 + fz], [1.0, 0.0], tex_index),
            Vertex::new([-0.5 + fx, 0.5 + fy, 0.5 + fz], [1.0, 1.0], tex_index),
            Vertex::new([-0.5 + fx, 0.5 + fy, -0.5 + fz], [0.0, 1.0], tex_index),
        ],
        Face::Right => vec![
            Vertex::new([0.5 + fx, -0.5 + fy, 0.5 + fz], [0.0, 0.0], tex_index),
            Vertex::new([0.5 + fx, -0.5 + fy, -0.5 + fz], [1.0, 0.0], tex_index),
            Vertex::new([0.5 + fx, 0.5 + fy, -0.5 + fz], [1.0, 1.0], tex_index),
            Vertex::new([0.5 + fx, 0.5 + fy, 0.5 + fz], [0.0, 1.0], tex_index),
        ],
        Face::Front => vec![
            Vertex::new([-0.5 + fx, -0.5 + fy, 0.5 + fz], [0.0, 0.0], tex_index),
            Vertex::new([0.5 + fx, -0.5 + fy, 0.5 + fz], [1.0, 0.0], tex_index),
            Vertex::new([0.5 + fx, 0.5 + fy, 0.5 + fz], [1.0, 1.0], tex_index),
            Vertex::new([-0.5 + fx, 0.5 + fy, 0.5 + fz], [0.0, 1.0], tex_index),
        ],
        Face::Back => vec![
            Vertex::new([0.5 + fx, -0.5 + fy, -0.5 + fz], [0.0, 0.0], tex_index),
            Vertex::new([-0.5 + fx, -0.5 + fy, -0.5 + fz], [1.0, 0.0], tex_index),
            Vertex::new([-0.5 + fx, 0.5 + fy, -0.5 + fz], [1.0, 1.0], tex_index),
            Vertex::new([0.5 + fx, 0.5 + fy, -0.5 + fz], [0.0, 1.0], tex_index),
        ],
    };

    // The indices are always the same pattern, offset by the base count
    let indices = [
        base_vertex_count + 0,
        base_vertex_count + 1,
        base_vertex_count + 2,
        base_vertex_count + 2,
        base_vertex_count + 3,
        base_vertex_count + 0,
    ];

    (verts, indices)
}
