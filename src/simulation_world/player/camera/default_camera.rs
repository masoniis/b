use crate::{
    prelude::*,
    simulation_world::{
        chunk::{ChunkCoord, CHUNK_SIDE_LENGTH},
        player::{ActiveCamera, CameraComponent},
    },
};
use bevy_ecs::prelude::*;

const DEFAULT_CAMERA_STARTING_X: f32 = (CHUNK_SIDE_LENGTH / 2) as f32;
const DEFAULT_CAMERA_STARTING_Y: f32 = 64.0;
const DEFAULT_CAMERA_STARTING_Z: f32 = (CHUNK_SIDE_LENGTH / 2) as f32;

/// A startup system that spawns a single default camera for a graphics project.
pub fn setup_camera_system(mut commands: Commands) {
    info!("Spawning default graphics camera.");

    let start_position = Vec3::new(
        DEFAULT_CAMERA_STARTING_X,
        DEFAULT_CAMERA_STARTING_Y,
        DEFAULT_CAMERA_STARTING_Z,
    );
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
