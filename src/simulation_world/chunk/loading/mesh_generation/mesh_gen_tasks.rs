use crate::prelude::*;
use crate::simulation_world::chunk::data_gen_tasks::CheckForBlockDataIsNoLongerNeeded;
use crate::simulation_world::chunk::mesh::TransparentMeshComponent;
use crate::simulation_world::chunk::ChunkState;
use crate::simulation_world::{
    asset_management::{texture_map_registry::TextureMapResource, AssetStorageResource, MeshAsset},
    block::BlockRegistryResource,
    chunk::{
        build_chunk_mesh, ChunkBlocksComponent, ChunkCoord, ChunkLoadingManager,
        OpaqueMeshComponent, TransformComponent, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH,
    },
};
use bevy_ecs::prelude::*;
use crossbeam::channel::{unbounded, Receiver, TryRecvError};

/// Marks a chunk meshing task in the simulation world that returns a MeshComponent.
#[derive(Component)]
pub struct ChunkMeshingTaskComponent {
    pub receiver: Receiver<(
        Option<OpaqueMeshComponent>,
        Option<TransparentMeshComponent>,
    )>,
}

/// A signal marking that chunks wants to be meshed. In this phase, the chunk is
/// waiting to be assigned to the thread pool, and can't be assigned until all
/// of its relevant neighbors have block data generated.
#[derive(Component)]
pub struct WantsMeshing;

/// A signal marking that chunks should be checked for meshing. This check is a necessary
/// optimization as chunks require all neighbors to be generated before they mesh.
#[derive(Component)]
pub struct CheckForMeshing;

/// Neighboring chunk data needed for meshing.
pub struct ChunkNeighborData {
    pub right: Option<ChunkBlocksComponent>,  // +X
    pub left: Option<ChunkBlocksComponent>,   // -X
    pub top: Option<ChunkBlocksComponent>,    // +Y
    pub bottom: Option<ChunkBlocksComponent>, // -Y
    pub front: Option<ChunkBlocksComponent>,  // +Z
    pub back: Option<ChunkBlocksComponent>,   // -Z
}

/// Queries for chunks needing meshing and starts a limited number of tasks per frame.
#[instrument(skip_all)]
pub fn start_pending_meshing_tasks_system(
    // Input
    mut pending_chunks_query: Query<
        (Entity, &ChunkBlocksComponent, &ChunkCoord),
        (
            With<WantsMeshing>,
            With<CheckForMeshing>,
            Without<ChunkMeshingTaskComponent>,
        ),
    >,
    all_generated_chunks: Query<&ChunkBlocksComponent>, // for finding neighbors

    // Resources needed to start meshing
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadingManager>,
    texture_map: Res<TextureMapResource>,
    block_registry: Res<BlockRegistryResource>,
    mesh_assets: Res<AssetStorageResource<MeshAsset>>,
) {
    'chunk_loop: for (entity, chunk_comp, chunk_coord) in pending_chunks_query.iter_mut() {
        // check for cancellation
        match chunk_manager.get_state(chunk_coord.pos) {
            Some(ChunkState::NeedsMeshing(state_entity)) if state_entity == entity => {
                // state is correct, proceed to start meshing
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Chunk {} marked NeedsMeshing but manager state is not NeedsMeshing({:?}). Assuming cancelled.",
                    chunk_coord.pos, entity
                );
                continue;
            }
        }

        // INFO: ----------------------------------------------
        //         Ensure neighbors have been generated
        // ----------------------------------------------------

        let get_neighbor = |offset: IVec3| -> Option<Option<ChunkBlocksComponent>> {
            let neighbor_coord = chunk_coord.pos + offset;
            match chunk_manager.get_entity(neighbor_coord) {
                Some(entity) => match all_generated_chunks.get(entity) {
                    Ok(blocks) => Some(Some(blocks.clone())), // found data
                    Err(_) => None,                           // must wait for generation
                },
                None => Some(None), // is out of bounds
            }
        };

        let right = match get_neighbor(IVec3::new(1, 0, 0)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let left = match get_neighbor(IVec3::new(-1, 0, 0)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let top = match get_neighbor(IVec3::new(0, 1, 0)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let bottom = match get_neighbor(IVec3::new(0, -1, 0)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let front = match get_neighbor(IVec3::new(0, 0, 1)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };
        let back = match get_neighbor(IVec3::new(0, 0, -1)) {
            Some(data) => data,
            None => {
                commands.entity(entity).remove::<CheckForMeshing>();
                continue 'chunk_loop;
            }
        };

        let neighbor_data_for_task = ChunkNeighborData {
            right,
            left,
            top,
            bottom,
            front,
            back,
        };

        trace!(target: "chunk_loading", "Starting meshing task for {}.", chunk_coord.pos);

        // INFO: -----------------------------
        //         Spawn the mesh task
        // -----------------------------------

        let texture_map_clone = texture_map.clone();
        let block_registry_clone = block_registry.clone();
        let mesh_assets_clone = mesh_assets.clone();
        let chunk_component_for_task = chunk_comp.clone();
        let coord_clone = chunk_coord.clone();

        let (sender, receiver) = unbounded();
        rayon::spawn(move || {
            let (opaque_mesh_option, transparent_mesh_option) = build_chunk_mesh(
                &coord_clone.to_string(),
                &chunk_component_for_task,
                &neighbor_data_for_task,
                &texture_map_clone,
                &block_registry_clone,
            );

            let omesh = if let Some(opaque_mesh) = opaque_mesh_option {
                let mesh_handle = mesh_assets_clone.add(opaque_mesh);
                Some(OpaqueMeshComponent::new(mesh_handle))
            } else {
                None
            };

            let tmesh = if let Some(transparent_mesh) = transparent_mesh_option {
                let mesh_handle = mesh_assets_clone.add(transparent_mesh);
                Some(TransparentMeshComponent::new(mesh_handle))
            } else {
                None
            };

            let _ = sender.send((omesh, tmesh));
        });

        // update entity and manager
        commands
            .entity(entity)
            .insert(ChunkMeshingTaskComponent { receiver })
            .remove::<CheckForMeshing>()
            .remove::<WantsMeshing>();

        chunk_manager.mark_as_meshing(chunk_coord.pos, entity);
    }
}

/// Polls chunk meshing tasks and adds the MeshComponent when ready.
#[instrument(skip_all)]
pub fn poll_chunk_meshing_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkMeshingTaskComponent, &ChunkCoord)>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadingManager>,
) {
    for (entity, meshing_task_component, coord) in tasks_query.iter_mut() {
        // check for cancellation
        match chunk_manager.get_state(coord.pos) {
            Some(ChunkState::Meshing(state_entity)) if state_entity == entity => {
                // state is correct, proceed
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Chunk meshing task for {} found but manager state is not Meshing({:?}). Assuming cancelled.",
                    coord, entity
                );
                continue;
            }
        }

        // poll mesh task
        match meshing_task_component.receiver.try_recv() {
            Ok((opaque_mesh_option, transparent_mesh_option)) => {
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

                // ping any neighbors that may be able to clear their data now
                for neighbor in chunk_manager.iter_neighbors(coord.pos) {
                    match neighbor.state {
                        ChunkState::Loaded(_) => {
                            commands
                                .entity(neighbor.entity)
                                .insert(CheckForBlockDataIsNoLongerNeeded);
                        }
                        _ => {}
                    }
                }

                // TODO: remoivng chunk blocks will save memory but if we remove
                // early then chunks next to it can't mesh so have to have a event
                // driven system for this probably
                // .remove::<ChunkCoord>()
                // .remove::<ChunkBlocksComponent>()

                chunk_manager.mark_as_loaded(coord.pos, entity);
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
