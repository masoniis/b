use crate::core::graphics::types::{TextureId, Vertex};
use crate::ecs_modules::rendering::{MeshComponent, TransformComponent};
use crate::ecs_resources::asset_storage::MeshAsset;
use crate::ecs_resources::texture_map::TextureMapResource;
use crate::ecs_resources::AssetStorageResource;
use bevy_ecs::prelude::{Commands, Res, ResMut};
use glam::Vec3;
use tracing::info;

pub fn cube_array_generation_system(
    mut commands: Commands,
    mut mesh_assets: ResMut<AssetStorageResource<MeshAsset>>,
    texture_map: Res<TextureMapResource>,
) {
    info!("Generating initial cube array...");

    let green_texture_index = texture_map.registry.get(TextureId::Stone);

    #[rustfmt::skip]
    let vertices_data: &[f32] = &[
        // Front face
        -0.5, -0.5, 0.5, 0.0, 0.0,
        0.5, -0.5, 0.5, 1.0, 0.0,
        0.5, 0.5, 0.5, 1.0, 1.0,
        -0.5, 0.5, 0.5, 0.0, 1.0,
        // Back face
        0.5, -0.5, -0.5, 0.0, 0.0,
        -0.5, -0.5, -0.5, 1.0, 0.0,
        -0.5, 0.5, -0.5, 1.0, 1.0,
        0.5, 0.5, -0.5, 0.0, 1.0,
    ];

    let vertices: Vec<Vertex> = vertices_data
        .chunks(5)
        .map(|chunk| {
            Vertex {
                position: [chunk[0], chunk[1], chunk[2]],
                color: [1.0, 1.0, 1.0], // Assuming white color
                tex_coords: [chunk[3], chunk[4]],
                texture_index: green_texture_index,
            }
        })
        .collect();

    let indices: [u32; 36] = [
        // Front face
        0, 1, 2, 2, 3, 0, // Back face
        4, 5, 6, 6, 7, 4, // Top face
        3, 2, 7, 7, 6, 3, // Bottom face
        5, 4, 1, 1, 0, 5, // Right face
        1, 4, 7, 7, 2, 1, // Left face
        5, 0, 3, 3, 6, 5,
    ];

    let cube_mesh_asset = MeshAsset {
        name: "cube_mesh".to_string(),
        vertices: vertices.clone(),
        indices: indices.to_vec(),
    };

    // 2. Add the asset to the central storage and get a handle.
    let mesh_handle = mesh_assets.add(cube_mesh_asset);

    // An array of cubes
    for x in 0..25 {
        for z in 0..25 {
            commands.spawn((
                MeshComponent::new(mesh_handle.clone()),
                TransformComponent {
                    position: Vec3::new((x * -2) as f32, 0.0, (z * -2) as f32),
                    ..Default::default()
                },
            ));
        }
    }
}
