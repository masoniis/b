use crate::ecs::components::{Mesh, Transform};
use bevy_ecs::prelude::*;
use glam::Vec3;
use tracing::info;

pub fn setup_chunk_system(mut commands: Commands) {
    info!("Generating initial chunk...");
    // A 16x16 chunk of simple blocks
    for x in 0..16 {
        for z in 0..16 {
            commands.spawn((
                Mesh::new_cube("src/assets/textures/grass_16x16.png"),
                Transform {
                    position: Vec3::new((x * 2) as f32, 0.0, (z * 2) as f32),
                    ..Default::default()
                },
            ));
        }
    }
}
