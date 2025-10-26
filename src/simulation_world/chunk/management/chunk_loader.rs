use crate::prelude::*;
use crate::simulation_world::{
    camera::ActiveCamera,
    chunk::{
        async_chunking::NeedsGenerating,
        load_manager::ChunkState,
        {load_manager::ChunkLoadManager, ChunkCoord},
        {RENDER_DISTANCE, WORLD_MAX_Y_CHUNK, WORLD_MIN_Y_CHUNK},
    },
};
use bevy_ecs::prelude::*;
use glam::IVec3;
use std::collections::HashSet;

/// Determines chunks to unload/load based on the camera position.
///
/// Only needs to run when the camera has entered a new chunk.
#[instrument(skip_all)]
pub fn manage_chunk_loading_system(
    // Input
    active_camera: Res<ActiveCamera>,
    camera_query: Query<&ChunkCoord, Changed<ChunkCoord>>,

    // Output
    mut chunk_manager: ResMut<ChunkLoadManager>, // for marking loaded/unloaded
    mut commands: Commands,                      // for spawning chunk entities
) {
    let Ok(camera_chunk) = camera_query.get(active_camera.0) else {
        return;
    };

    // calculate desired chunks based on render distance
    let camera_chunk_pos = camera_chunk.pos;
    let mut desired_chunks = HashSet::new();
    for y in WORLD_MIN_Y_CHUNK..=WORLD_MAX_Y_CHUNK {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let coord = IVec3::new(camera_chunk_pos.x + x, y, camera_chunk_pos.z + z);
                desired_chunks.insert(coord);
            }
        }
    }

    // INFO: --------------------------------
    //         unload/cancel chunking
    // --------------------------------------

    let mut coords_to_remove = Vec::new();

    // iterate through all currently tracked chunks
    for (coord, state) in chunk_manager.chunk_states.iter() {
        // if chunk is out of range...
        if !desired_chunks.contains(coord) {
            match state {
                ChunkState::NeedsGenerating(entity)
                | ChunkState::Generating(entity)
                | ChunkState::NeedsMeshing(entity)
                | ChunkState::Meshing(entity) => {
                    debug!(target:"chunk_loading", "Cancelling task for chunk at {:?} (Entity: {:?})", coord, entity);
                    commands.entity(*entity).despawn();
                }
                ChunkState::Loaded(entity) => {
                    debug!(target:"chunk_loading", "Unloading loaded chunk at {:?} (Entity: {:?})", coord, entity);
                    commands.entity(*entity).despawn();
                }
            }
            coords_to_remove.push(*coord);
        }
    }

    // remove the unloaded/cancelled chunks from the manager
    for coord in coords_to_remove {
        chunk_manager.mark_as_unloaded(coord);
    }

    // INFO: --------------------------------------------
    //         load new chunks (start generation)
    // --------------------------------------------------

    for coord in desired_chunks {
        // mark as needing generation if it is not already being loaded
        if !chunk_manager.is_chunk_present_or_loading(coord) {
            debug!(target:"chunk_loading","Marking chunk needs-generation at {:?}", coord);
            let ent = commands
                .spawn((NeedsGenerating, ChunkCoord { pos: coord }))
                .id();
            chunk_manager.mark_as_needs_generating(coord, ent);
        }
    }
}
