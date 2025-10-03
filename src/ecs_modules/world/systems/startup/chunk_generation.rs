use crate::core::graphics::types::{TextureId, Vertex};
use crate::ecs_modules::graphics::{MeshComponent, TransformComponent};
use crate::ecs_modules::world::utils::types::chunk::Chunk;
use crate::ecs_modules::world::world_gen::{generate_flat_world_chunk, CHUNK_HEIGHT};
use crate::ecs_resources::asset_storage::AssetStorageResource;
use crate::ecs_resources::asset_storage::MeshAsset;
use crate::ecs_resources::texture_map::TextureMapResource;
use bevy_ecs::prelude::{Commands, Res, ResMut};
use glam::Vec3;
use tracing::info;

pub fn chunk_generation_system(
    mut commands: Commands,
    mut mesh_assets: ResMut<AssetStorageResource<MeshAsset>>,
    texture_map: Res<TextureMapResource>,
) {
    info!("Generating initial chunk...");

    let missing_texture_index = texture_map.registry.get(TextureId::Missing);

    let mut chunk = Chunk::new(0, 0, 0);
    generate_flat_world_chunk(&mut chunk);

    // Temporary verification: print some block IDs
    info!(
        "Block at (0, 0, 0): {:?}",
        chunk.get_block(0, 0, 0).map(|b| b.id)
    );
    info!(
        "Block at (0, {}, 0): {:?}",
        CHUNK_HEIGHT - 2,
        chunk.get_block(0, CHUNK_HEIGHT - 2, 0).map(|b| b.id)
    );
    info!(
        "Block at (0, {}, 0): {:?}",
        CHUNK_HEIGHT - 1,
        chunk.get_block(0, CHUNK_HEIGHT - 1, 0).map(|b| b.id)
    );

    // Create a simple cube mesh with hardcoded vertices, colors, and UVs
    #[rustfmt::skip]
    let vertices = vec![
        // Front face
        Vertex { position: [-0.5, -0.5, 0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5, -0.5, 0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5,  0.5, 0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0], texture_index: missing_texture_index },
        Vertex { position: [-0.5,  0.5, 0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0], texture_index: missing_texture_index },

        // Back face
        Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0], texture_index: missing_texture_index },
        Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0], texture_index: missing_texture_index },

        // Top face
        Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0], texture_index: missing_texture_index },
        Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0], texture_index: missing_texture_index },

        // Bottom face
        Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0], texture_index: missing_texture_index },
        Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0], texture_index: missing_texture_index },

        // Right face
        Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0], texture_index: missing_texture_index },
        Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0], texture_index: missing_texture_index },

        // Left face
        Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], texture_index: missing_texture_index },
        Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0], texture_index: missing_texture_index },
        Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0], texture_index: missing_texture_index },
    ];

    #[rustfmt::skip]
    let indices: Vec<u32> = vec![
        0,  1,  2,  2,  3,  0,  // Front
        4,  5,  6,  6,  7,  4,  // Back
        8,  9, 10, 10, 11,  8,  // Top
        12, 13, 14, 14, 15, 12, // Bottom
        16, 17, 18, 18, 19, 16, // Right
        20, 21, 22, 22, 23, 20, // Left
    ];

    let cube_mesh_asset = MeshAsset {
        vertices,
        indices,
        name: "cube_mesh2".to_string(),
    };
    let mesh_handle = mesh_assets.add(cube_mesh_asset);

    // Spawn a single entity with the cube mesh and a transform
    commands.spawn((
        MeshComponent::new(mesh_handle),
        TransformComponent {
            position: Vec3::new(0.0, -2.0, 0.0),
            ..Default::default()
        },
    ));
}
