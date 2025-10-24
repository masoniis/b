use crate::prelude::*;
use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::camera::ActiveCamera;
use crate::simulation_world::chunk::async_chunking::ChunkGenerationTaskComponent;
use crate::simulation_world::chunk::superflat_generator::SuperflatGenerator;
use crate::simulation_world::chunk::ChunkGenerator;
use crate::simulation_world::chunk::{load_manager::ChunkLoadManager, ChunkChord};
use bevy_ecs::prelude::*;
use bevy_tasks::AsyncComputeTaskPool;
use futures_timer::Delay;
use glam::IVec3;
use rand::Rng;
use std::collections::HashSet;
use std::time::Duration;

/// The distance, in chunks, to load around the camera.
const RENDER_DISTANCE: i32 = 11;
const VERTICAL_RENDER_DISTANCE: i32 = 1;

/// Determines chunks to unload/load based on the camera position.
///
/// Only needs to run when the camera has entered a new chunk.
#[instrument(skip_all)]
pub fn manage_chunk_loading_system(
    // Input
    active_camera: Res<ActiveCamera>,
    block_registry: Res<BlockRegistryResource>,
    camera_query: Query<&ChunkChord, Changed<ChunkChord>>,

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
            debug!(target:"chunk_loading","Unloading loaded chunk at {:?} (Entity: {:?})", coord, entity);

            commands.entity(*entity).despawn();

            false
        }
    });

    // cancel generation pass
    chunk_manager.generating_chunks.retain(|coord, entity| {
        if desired_chunks.contains(coord) {
            true // chunk in range, keep it
        } else {
            debug!(target:"chunk_loading","Unloading currently-generating chunk at {:?} (Entity: {:?})", coord, entity);

            commands.entity(*entity).despawn();

            false
        }
    });

    chunk_manager.meshing_chunks.retain(|coord, entity| {
        if desired_chunks.contains(coord) {
            true // chunk in range, keep it
        } else {
            debug!(target:"chunk_loading","Unloading currently-meshing chunk at {:?} (Entity: {:?})", coord, entity);

            commands.entity(*entity).despawn();

            false
        }
    });

    // load in chunks
    let task_pool = AsyncComputeTaskPool::get();
    for coord in desired_chunks {
        if !chunk_manager.is_chunk_present_or_loading(coord) {
            debug!(target:"chunk_loading","Requesting chunk load at {:?}", coord);

            let blocks = block_registry.clone();

            let task = task_pool.spawn(async move {
                let gen = SuperflatGenerator::new();
                // TODO: not having a random delay causes frame skips, I am not 100%
                // sure on why but I assume it throttles the OS by adding so many
                // chunks at once. Need to create some system that ensures we only
                // begin meshing on a few chunks per frame to stop this from occurring.
                let duration = Duration::from_secs_f32(rand::rng().random_range(0.05..1.0));
                Delay::new(duration).await;
                return gen.generate_chunk(coord.clone(), &blocks);
            });

            let ent = commands
                .spawn(ChunkGenerationTaskComponent {
                    task: task,
                    coord: coord,
                })
                .id();

            chunk_manager.mark_as_generating(coord, ent);
        }
    }
}
