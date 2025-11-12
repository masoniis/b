use crate::prelude::*;
use crate::simulation_world::chunk::{
    CheckForMeshing, ChunkCoord, ChunkMeshingTaskComponent, ChunkState, ChunkStateManager,
    OpaqueMeshComponent, TransformComponent, TransparentMeshComponent, WantsMeshing,
    CHUNK_SIDE_LENGTH,
};
use bevy_ecs::prelude::*;
use crossbeam::channel::TryRecvError;

/// Polls chunk meshing tasks and adds the MeshComponent when ready.
#[instrument(skip_all)]
pub fn poll_chunk_meshing_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkMeshingTaskComponent, &ChunkCoord)>,
    existing_meshes: Query<(
        Option<&OpaqueMeshComponent>,
        Option<&TransparentMeshComponent>,
    )>,

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
                    Some(
                        ChunkState::Meshing { entity: gen_entity }
                        | ChunkState::WantsMeshing { entity: gen_entity },
                    ) if gen_entity == entity => {
                        trace!(target : "chunk_loading","Chunk meshing finished for {:?}", coord);

                        let (exists_opaque, exists_transparent) =
                            existing_meshes.get(entity).unwrap_or((None, None));

                        if exists_opaque.is_some() {
                            commands.entity(entity).remove::<OpaqueMeshComponent>();
                        }
                        if exists_transparent.is_some() {
                            commands.entity(entity).remove::<TransparentMeshComponent>();
                        }

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
                                // TODO: this is a bug. A all stone mesh in a  mountain will get no mesh
                                // and then get despawned. then a chunk next to it will see it is "loaded but empty" and
                                // assume air. This results in interior faces being meshed.
                                warn!("Both opaque and transparent meshes are empty for chunk at {:?} after meshing, but typically this should be avoided by despawning the entity after generation to avoid meshing entirely. Despawning entity now.", coord);

                                commands
                                    .entity(entity)
                                    .remove::<ChunkMeshingTaskComponent>();
                                chunk_manager.mark_as_loaded(coord.pos, entity);
                                return; // return to avoid adding transform component
                            }
                        }

                        commands
                            .entity(entity)
                            .insert(TransformComponent {
                                position: Vec3::new(
                                    (coord.x * CHUNK_SIDE_LENGTH as i32) as f32,
                                    (coord.y * CHUNK_SIDE_LENGTH as i32) as f32,
                                    (coord.z * CHUNK_SIDE_LENGTH as i32) as f32,
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
