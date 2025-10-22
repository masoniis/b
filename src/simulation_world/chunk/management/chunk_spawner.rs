use crate::prelude::*;
use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::superflat_generator::SuperflatGenerator;
use crate::simulation_world::chunk::ChunkGenerator;
use crate::simulation_world::{
    camera::camera::ActiveCamera,
    chunk::{load_manager::ChunkLoadManager, ChunkChord},
};
use bevy_ecs::prelude::*;
use glam::IVec3;
use std::collections::HashSet;
use tracing::info;

/// The distance, in chunks, to load around the camera.
const RENDER_DISTANCE: i32 = 1;
const VERTICAL_RENDER_DISTANCE: i32 = 1;

/// Determines chunks to unload/load based on the camera position.
///
/// Only needs to run when the camera has entered a new chunk.
#[instrument(skip_all)]
pub fn manage_chunk_loading_system(
    // Input
    active_camera: Res<ActiveCamera>,
    blocks: Res<BlockRegistryResource>,
    camera_query: Query<&ChunkChord, Changed<ChunkChord>>,

    // Output
    mut chunk_manager: ResMut<ChunkLoadManager>, // for marking loaded/unloaded
    mut commands: Commands,                      // for spawning chunk entities
) {
    let Ok(camera_chunk) = camera_query.get(active_camera.0) else {
        return;
    };

    let camera_chunk_pos = camera_chunk.pos;
    info!("Camera moved to new chunk: {:?}", camera_chunk_pos);

    // calculate desired chunks based on render distance
    let mut desired_chunks = HashSet::new();
    for y in -VERTICAL_RENDER_DISTANCE..=VERTICAL_RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let coord = camera_chunk_pos + IVec3::new(x, y, z);
                desired_chunks.insert(coord);
            }
        }
    }

    // unload pass
    chunk_manager.loaded_chunks.retain(|coord, entity| {
        if desired_chunks.contains(coord) {
            true // chunk in range, keep it
        } else {
            info!("Unloading chunk at {:?} (Entity: {:?})", coord, entity);

            commands.entity(*entity).despawn();

            // TODO: consider removing from loading set as well
            // chunk_manager.loading_chunks.remove(coord);
            false
        }
    });

    // load in chunks
    let gen = SuperflatGenerator::new();
    for coord in desired_chunks {
        if !chunk_manager.is_chunk_present_or_loading(coord) {
            info!("Requesting chunk load at {:?}", coord);
            chunk_manager.mark_as_loading(coord);

            let chunk = gen.generate_chunk(coord, &blocks);
            let ent = commands.spawn((chunk.chunk, chunk.transform)).id();

            chunk_manager.mark_as_loaded(coord, ent);
        }
    }
}
