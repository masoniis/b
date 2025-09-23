use crate::ecs::components::{MeshComponent, TransformComponent};
use crate::ecs::resources::TextureManagerResource;
use crate::graphics::textures::Texture;
use bevy_ecs::prelude::{Commands, NonSendMut};
use glam::{Vec2, Vec3};
use tracing::info;

pub fn chunk_generation_system(
    mut commands: Commands,
    mut texture_manager: NonSendMut<TextureManagerResource>,
) {
    info!("Generating initial chunk...");

    // Load the grass texture once and add it to the TextureManager
    let grass_texture =
        Texture::new("src/assets/textures/missing.png").expect("Failed to load grass texture");
    texture_manager.add_atlas("main_atlas".to_string(), grass_texture);

    let atlas_id = "main_atlas".to_string();
    let uv_min = Vec2::new(0.0, 0.0);
    let uv_max = Vec2::new(1.0, 1.0);

    // An array of cubes
    for x in 0..100 {
        for z in 0..100 {
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
