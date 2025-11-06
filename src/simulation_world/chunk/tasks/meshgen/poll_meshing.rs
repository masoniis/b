use crate::prelude::*;
use crate::simulation_world::chunk::{
    CheckForMeshing, ChunkMeshingTaskComponent, ChunkState, WantsMeshing,
};
use crate::simulation_world::chunk::{
    ChunkCoord, ChunkStateManager, TransformComponent, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH,
};
use bevy_ecs::prelude::*;
use crossbeam::channel::TryRecvError;

/// Polls chunk meshing tasks and adds the MeshComponent when ready.
#[instrument(skip_all)]
pub fn poll_chunk_meshing_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkMeshingTaskComponent, &ChunkCoord)>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkStateManager>,
) {
    // poll all mesh task
    for (entity, meshing_task_component, coord) in tasks_query.iter_mut() {
        match meshing_task_component.receiver.try_recv() {
            Ok((opaque_mesh_option, transparent_mesh_option)) => {
                let current_state = chunk_manager.get_state(coord.pos);
                match current_state {
                    Some(ChunkState::Meshing(gen_entity)) if gen_entity == entity => {
                        trace!(target : "chunk_loading","Chunk meshing finished for {:?}", coord);

                        match (opaque_mesh_option, transparent_mesh_option) {
                            (Some(opaque_mesh), Some(transparent_mesh)) => {
                                commands
                                    .entity(entity)
                                    .insert((opaque_mesh, transparent_mesh));
                            }
                            (Some(opaque_mesh), None) => {
                                commands.entity(entity).insert(opaque_mesh);
                            }
                            (None, Some(transparent_mesh)) => {
                                commands.entity(entity).insert(transparent_mesh);
                            }
                            (None, None) => {
                                warn!("Both opaque and transparent meshes are empty for chunk at {:?} after meshing, but typically this should be avoided by despawning the entity after generation to avoid meshing entirely. Despawning entity now.", coord);
                                commands.entity(entity).despawn();
                                chunk_manager.mark_as_loaded_but_empty(coord.pos);
                                return; // return to avoid adding transform component
                            }
                        }

                        commands
                            .entity(entity)
                            .insert(TransformComponent {
                                position: Vec3::new(
                                    (coord.x * CHUNK_WIDTH as i32) as f32,
                                    (coord.y * CHUNK_HEIGHT as i32) as f32,
                                    (coord.z * CHUNK_DEPTH as i32) as f32,
                                ),
                                rotation: Quat::IDENTITY,
                                scale: Vec3::ONE,
                            })
                            .remove::<ChunkMeshingTaskComponent>();

                        chunk_manager.mark_as_loaded(coord.pos, entity);
                    }
                    Some(_) => {
                        error!(
                            "Chunk meshing task for {} completed but manager state entity does not match ({:?} != {:?}).",
                            coord, current_state.unwrap().entity(), entity
                        );
                    }
                    _ => {
                        debug!(
                            target : "chunk_loading",
                            "Mesh generation completed for unloaded chunk coord {}. Cleaning up entity {}.",
                            coord, entity
                        );
                        commands
                            .entity(entity)
                            .remove::<ChunkMeshingTaskComponent>();
                        continue;
                    }
                }
            }
            Err(TryRecvError::Empty) => {
                // task still running
            }
            Err(TryRecvError::Disconnected) => {
                warn!(
                    target: "chunk_loading",
                    "Chunk meshing task for {} failed (channel disconnected). Despawning entity.",
                    coord
                );
                // try to send it to be remeshed
                chunk_manager.mark_as_needs_meshing(coord.pos, entity);
                commands
                    .entity(entity)
                    .remove::<ChunkMeshingTaskComponent>()
                    .insert(CheckForMeshing)
                    .insert(WantsMeshing);
            }
        }
    }
}
