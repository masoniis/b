use crate::ecs::components::{MeshComponent, TransformComponent};
use bevy_ecs::prelude::Commands;
use glam::{Vec2, Vec3};
use tracing::info;

pub fn chunk_generation_system(mut commands: Commands) {
    info!("Generating initial chunk...");

    let atlas_id = "main_atlas".to_string(); // This will be unused for now
    let uv_min = Vec2::new(0.0, 0.0);
    let uv_max = Vec2::new(1.0, 1.0);

    // An array of cubes
    for x in 0..50 {
        for z in 0..50 {
            commands.spawn((
                MeshComponent::new_cube(atlas_id.clone(), uv_min, uv_max),
                TransformComponent {
                    position: Vec3::new((x * -2) as f32, 0.0, (z * -2) as f32),
                    ..Default::default()
                },
            ));
        }
    }
}
