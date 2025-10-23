use crate::prelude::*;
use crate::simulation_world::asset_management::MeshComponentRemovedMessage;
use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::superflat_generator::SuperflatGenerator;
use crate::simulation_world::chunk::{ChunkGenerator, MeshComponent};
use crate::simulation_world::{
    camera::camera::ActiveCamera,
    chunk::{load_manager::ChunkLoadManager, ChunkChord},
};
use bevy_ecs::prelude::*;
use glam::IVec3;
use std::collections::HashSet;

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
    mesh_query: Query<&MeshComponent>, // to get handles on despawn

    // Output
    mut chunk_manager: ResMut<ChunkLoadManager>, // for marking loaded/unloaded
    mut commands: Commands,                      // for spawning chunk entities
    mut mesh_removed_writer: MessageWriter<MeshComponentRemovedMessage>,
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
            debug!(target:"chunk_loading","Unloading chunk at {:?} (Entity: {:?})", coord, entity);

            // if the chunk entity had a mesh component, send a removal message
            //
            // (it is common that an air chunk exists with no mesh)
            if let Ok(mesh_component) = mesh_query.get(*entity) {
                mesh_removed_writer.write(MeshComponentRemovedMessage {
                    mesh_handle: mesh_component.mesh_handle,
                });
            }

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
            debug!(target:"chunk_loading","Requesting chunk load at {:?}", coord);
            chunk_manager.mark_as_loading(coord);

            let chunk = gen.generate_chunk(coord, &blocks);
            let ent = commands.spawn((chunk.chunk, chunk.transform)).id();

            chunk_manager.mark_as_loaded(coord, ent);
        }
    }
}
