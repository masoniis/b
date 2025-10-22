use crate::{
    prelude::*,
    simulation_world::{
        camera::{ActiveCamera, CameraComponent},
        chunk::{chunk_chord::world_to_chunk_pos, ChunkChord},
    },
};
use bevy_ecs::prelude::*;

/// A startup system that spawns a single default camera for a graphics project.
pub fn setup_camera_system(mut commands: Commands) {
    info!("Spawning default graphics camera.");

    let start_position = Vec3::new(0.0, 5.0, 0.0);
    let start_chunk = world_to_chunk_pos(start_position);

    let camera_entity = commands
        .spawn((
            CameraComponent {
                position: start_position,
                ..Default::default()
            },
            ChunkChord { pos: start_chunk },
        ))
        .id();

    commands.insert_resource(ActiveCamera(camera_entity));
}
