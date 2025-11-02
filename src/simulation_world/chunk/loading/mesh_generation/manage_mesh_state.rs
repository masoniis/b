use crate::prelude::*;
use crate::simulation_world::chunk::mesh_gen_tasks::WantsMeshing;
use crate::simulation_world::chunk::{ChunkCoord, ChunkLoadingManager, ChunkState};
use crate::simulation_world::{
    camera::ActiveCamera,
    chunk::{RENDER_DISTANCE, WORLD_MAX_Y_CHUNK, WORLD_MIN_Y_CHUNK},
};
use bevy_ecs::prelude::*;
use glam::IVec3;
use std::collections::HashSet;

/// Assesses whether a chunk coordinate is within the meshing radius of the camera.
pub fn chunk_is_in_mesh_radius(camera_chunk_pos: IVec3, chunk_coord: IVec3) -> bool {
    let dx = chunk_coord.x - camera_chunk_pos.x;
    let dy = chunk_coord.y;
    let dz = chunk_coord.z - camera_chunk_pos.z;

    dx.abs() <= RENDER_DISTANCE
        && dy >= WORLD_MIN_Y_CHUNK
        && dy <= WORLD_MAX_Y_CHUNK
        && dz.abs() <= RENDER_DISTANCE
}

/// Promotes/demotes chunks between "DataReady" and "Loaded" (meshed) states.
///
/// This system should only run when the camera moves to a new chunk.
#[instrument(skip_all)]
pub fn manage_chunk_meshing_system(
    // Input
    active_camera: Res<ActiveCamera>,
    camera_query: Query<&ChunkCoord>,

    // Output
    mut chunk_manager: ResMut<ChunkLoadingManager>,
    mut commands: Commands,
) {
    let camera_chunk = camera_query.get(active_camera.0).unwrap();

    let camera_chunk_pos = camera_chunk.pos;
    let mut desired_mesh_chunks = HashSet::new();
    for y in WORLD_MIN_Y_CHUNK..=WORLD_MAX_Y_CHUNK {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let coord = IVec3::new(camera_chunk_pos.x + x, y, camera_chunk_pos.z + z);
                desired_mesh_chunks.insert(coord);
            }
        }
    }

    // promote/demote chunks based on render distance
    let mut to_demote = vec![];
    for (coord, state) in chunk_manager.chunk_states.iter_mut() {
        let is_in_mesh_radius = desired_mesh_chunks.contains(coord);

        match state {
            // mesh if data ready and in radius
            ChunkState::DataReady(entity) => {
                if is_in_mesh_radius {
                    debug!(target:"chunk_meshing", "Promoting chunk {:?} to NeedsMeshing", coord);
                    commands.entity(*entity).insert(WantsMeshing);
                    *state = ChunkState::NeedsMeshing(*entity);
                }
            }

            // demesh if meshing/meshed and out of radius
            ChunkState::NeedsMeshing(entity)
            | ChunkState::Meshing(entity)
            | ChunkState::Loaded(Some(entity)) => {
                if !is_in_mesh_radius {
                    debug!(target:"chunk_meshing", "Demoting chunk {:?} to DataReady", coord);
                    to_demote.push((*coord, *entity));
                }
            }

            // leave alone if still needs generation or generating
            ChunkState::NeedsGenerating(_)
            | ChunkState::Generating(_)
            | ChunkState::Loaded(None) => {}
        }
    }

    for (coord, entity) in to_demote {
        commands.entity(entity).despawn();
        chunk_manager.mark_as_unloaded(coord);
    }
}
