use crate::ecs::components::{MeshComponent, TransformComponent, create_gpu_mesh_from_data};
use crate::graphics::Vertex;
use crate::graphics::webgpu_renderer::WebGpuRenderer;
use bevy_ecs::prelude::Commands;
use bevy_ecs::prelude::ResMut;
use glam::{Vec2, Vec3};
use tracing::info;

pub fn chunk_generation_system(mut commands: Commands, renderer: ResMut<WebGpuRenderer>) {
    info!("Generating initial chunk...");

    let atlas_id = "main_atlas".to_string(); // This will be unused for now
    let uv_min = Vec2::new(0.0, 0.0);
    let uv_max = Vec2::new(1.0, 1.0);

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

    let gpu_mesh = create_gpu_mesh_from_data(renderer.get_device(), &vertices, &indices);

    // An array of cubes
    for x in 0..100 {
        for z in 0..100 {
            commands.spawn((
                MeshComponent::new(&gpu_mesh, atlas_id.clone(), uv_min, uv_max),
                TransformComponent {
                    position: Vec3::new((x * -2) as f32, 0.0, (z * -2) as f32),
                    ..Default::default()
                },
            ));
        }
    }
}
