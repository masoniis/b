use crate::{
    prelude::*,
    simulation_world::{
        chunk::{ChunkCoord, CHUNK_DEPTH, CHUNK_WIDTH},
        player::{ActiveCamera, CameraComponent},
    },
};
use bevy_ecs::prelude::*;

/// A startup system that spawns a single default camera for a graphics project.
pub fn setup_camera_system(mut commands: Commands) {
    info!("Spawning default graphics camera.");

    let start_position = Vec3::new((CHUNK_WIDTH / 2) as f32, 40.0, (CHUNK_DEPTH / 2) as f32);
    let start_chunk = ChunkCoord::world_to_chunk_pos(start_position);

    let camera_entity = commands
        .spawn((
            CameraComponent {
                position: start_position,
                ..Default::default()
            },
            ChunkCoord { pos: start_chunk },
        ))
        .id();

    commands.insert_resource(ActiveCamera(camera_entity));
}
