use crate::prelude::*;
use crate::simulation_world::chunk::{ChunkCoord, ChunkLoadingManager, ChunkState};
use crate::simulation_world::{
    camera::ActiveCamera,
    chunk::{
        data_gen_tasks::NeedsGenerating,
        {LOAD_DISTANCE, WORLD_MAX_Y_CHUNK, WORLD_MIN_Y_CHUNK},
    },
};
use bevy_ecs::prelude::*;
use glam::IVec3;
use std::collections::HashSet;

/// Determines chunks to unload/load based on the camera position and LOAD_DISTANCE.
///
/// Only needs to run when the camera has entered a new chunk.
#[instrument(skip_all)]
pub fn manage_chunk_loading_system(
    // Input
    active_camera: Res<ActiveCamera>,
    camera_query: Query<&ChunkCoord>,

    // Output
    mut chunk_manager: ResMut<ChunkLoadingManager>, // for marking loaded/unloaded
    mut commands: Commands,                         // for spawning chunk entities
) {
    let camera_chunk = camera_query.get(active_camera.0).unwrap();

    // calculate desired chunks based on LOAD distance (render distance + 1)
    let camera_chunk_pos = camera_chunk.pos;
    let mut desired_load_chunks = HashSet::new();
    for y in WORLD_MIN_Y_CHUNK..=WORLD_MAX_Y_CHUNK {
        for z in -LOAD_DISTANCE..=LOAD_DISTANCE {
            for x in -LOAD_DISTANCE..=LOAD_DISTANCE {
                let coord = IVec3::new(camera_chunk_pos.x + x, y, camera_chunk_pos.z + z);
                desired_load_chunks.insert(coord);
            }
        }
    }

    // INFO: --------------------------------
    //         unload/cancel chunking
    // --------------------------------------

    // iterate through all currently tracked chunks and despawn those that are no longer desired
    let mut coords_to_remove = Vec::new();
    for (coord, state) in chunk_manager.chunk_states.iter() {
        if !desired_load_chunks.contains(coord) {
            match state {
                ChunkState::NeedsGenerating(entity)
                | ChunkState::Generating(entity)
                | ChunkState::DataReady(entity)
                | ChunkState::NeedsMeshing(entity)
                | ChunkState::Meshing(entity)
                | ChunkState::Loaded(Some(entity)) => {
                    debug!(target:"chunk_loading", "Unloading chunk at {:?} (Entity: {:?})", coord, entity);
                    commands.entity(*entity).despawn();
                }
                ChunkState::Loaded(None) => {
                    // already unloaded, nothing to despawn
                    debug!(target:"chunk_loading", "Marking chunk at {:?} as unloaded (was already unloaded)", coord);
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

    // if any desired chunks are not currently loaded or loading, spawn a new chunk entity and mark it as needs-generation
    for coord in desired_load_chunks {
        if !chunk_manager.is_chunk_present_or_loading(coord) {
            debug!(target:"chunk_loading","Marking chunk needs-generation at {:?}", coord);
            let ent = commands
                .spawn((NeedsGenerating, ChunkCoord { pos: coord }))
                .id();
            chunk_manager.mark_as_needs_generating(coord, ent);
        }
    }
}
