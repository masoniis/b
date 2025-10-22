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
const RENDER_DISTANCE: i32 = 2;
/// The vertical distance, in chunks, to load.
const VERTICAL_RENDER_DISTANCE: i32 = 1;

/// This system is the "brain" of chunk management.
///
/// It runs when the camera's `ChunkVicinity` changes, calculating
/// which chunks to load and which to unload.
#[instrument(skip_all)]
pub fn manage_chunk_loading_system(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
    active_camera: Res<ActiveCamera>,
    blocks: Res<BlockRegistryResource>,
    // This query is the key:
    // It now gets ALL entities that have a changed ChunkChord.
    camera_query: Query<&ChunkChord, Changed<ChunkChord>>,
) {
    let camera_entity = active_camera.0;

    // 2. Try to get the ChunkChord *for that specific entity*.
    // This will only return Ok(..) if the camera_entity exists
    // AND has a ChunkChord component AND that component has Changed.
    let Ok(camera_vicinity) = camera_query.get(camera_entity) else {
        return; // No change on our active camera, no work to do.
    };
    let camera_chunk_pos = camera_vicinity.pos;
    info!("Camera moved to new chunk: {:?}", camera_chunk_pos);

    // 3. Calculate the new set of desired chunks (Unchanged)
    let mut desired_chunks = HashSet::new();
    for y in -VERTICAL_RENDER_DISTANCE..=VERTICAL_RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
                let coord = camera_chunk_pos + IVec3::new(x, y, z);
                desired_chunks.insert(coord);
            }
        }
    }

    // Unload Pass
    chunk_manager.loaded_chunks.retain(|coord, entity| {
        if desired_chunks.contains(coord) {
            // This chunk is still in range, keep it.
            true
        } else {
            // This chunk is out of range. Despawn it.
            info!("Unloading chunk at {:?} (Entity: {:?})", coord, entity);

            // --- THIS IS THE FIX ---
            commands.entity(*entity).despawn();
            // -----------------------

            // Also remove it from the loading set, just in case.
            // chunk_manager.loading_chunks.remove(coord);
            // Return false to remove it from the loaded_chunks map.
            false
        }
    });

    // 5. Load Pass (Unchanged)
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
