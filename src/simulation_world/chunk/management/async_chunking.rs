use crate::prelude::*;
use crate::simulation_world::asset_management::texture_map_registry::TextureMapResource;
use crate::simulation_world::asset_management::AssetStorageResource;
use crate::simulation_world::asset_management::MeshAsset;
use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::chunk_meshing::build_chunk_mesh;
use crate::simulation_world::chunk::load_manager::ChunkLoadManager;
use crate::simulation_world::chunk::GeneratedChunkComponents;
use crate::simulation_world::chunk::MeshComponent;
use bevy_ecs::prelude::Component;
use bevy_ecs::prelude::*;
use bevy_tasks::futures::now_or_never;
use bevy_tasks::AsyncComputeTaskPool;
use bevy_tasks::Task;
use futures_timer::Delay;
use glam::IVec3;
use rand::Rng;
use std::time::Duration;

/// Marks a chunk loading task in the simulation world that returns nothing.
#[derive(Component)]
pub struct ChunkGenerationTaskComponent {
    pub task: Task<GeneratedChunkComponents>,
    pub coord: IVec3,
}

/// Marks a chunk meshing task in the simulation world that returns a MeshComponent.
#[derive(Component)]
pub struct ChunkMeshingTaskComponent {
    pub task: Task<Option<MeshComponent>>,
    pub coord: IVec3,
}

/// Polls all chunk generation tasks and spawns in the resulting chunk data when ready.
///
/// Dispatches a mesh task.
pub fn poll_chunk_generation_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkGenerationTaskComponent)>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,

    // Resources needed for meshing
    texture_map: Res<TextureMapResource>,
    block_registry: Res<BlockRegistryResource>,
    mesh_assets: Res<AssetStorageResource<MeshAsset>>,
) {
    for (entity, mut generation_task_component) in tasks_query.iter_mut() {
        let coord = generation_task_component.coord;

        // check for cancellation from loader to avoid polling a to-be-killed entity
        // which could lead to race conditions.
        if !chunk_manager.generating_chunks.contains_key(&coord) {
            warn!(
                "Chunk generation task for {} was cancelled before completion.",
                coord
            );
            commands
                .entity(entity)
                .remove::<ChunkGenerationTaskComponent>();
            continue;
        }

        // poll the generation task
        if let Some(generated_data) = now_or_never(&mut generation_task_component.task) {
            debug!(
                target: "chunk_loading",
                "Chunk generation finished for {}. Starting meshing task.",
                coord
            );

            // INFO: --------------------------------------------
            //         Generation complete, start meshing
            // --------------------------------------------------

            // clone resources needed for meshing thread
            let texture_map_clone = texture_map.clone();
            let block_registry_clone = block_registry.clone();
            let mesh_assets_clone = mesh_assets.clone();

            let task_pool = AsyncComputeTaskPool::get();
            let meshing_task_handle: Task<Option<MeshComponent>> = task_pool.spawn(async move {
                let (vertices, indices) = build_chunk_mesh(
                    &generated_data.chunk_component,
                    &texture_map_clone,
                    &block_registry_clone,
                );

                let duration = Duration::from_secs_f32(rand::rng().random_range(0.05..1.0));
                Delay::new(duration).await;

                if !vertices.is_empty() {
                    let mesh_asset = MeshAsset {
                        name: format!("chunk_{}_{}_{}", coord.x, coord.y, coord.z),
                        vertices,
                        indices,
                    };
                    let mesh_handle = mesh_assets_clone.add(mesh_asset);
                    Some(MeshComponent::new(mesh_handle))
                } else {
                    None
                }
            });

            // append the meshing task to the entity and remove generation
            commands
                .entity(entity)
                .remove::<ChunkGenerationTaskComponent>()
                .insert((
                    generated_data.transform_component,
                    (ChunkMeshingTaskComponent {
                        task: meshing_task_handle,
                        coord,
                    }),
                ));

            // mark as meshing now (same entity from generation)
            chunk_manager.mark_as_meshing(coord, entity);
        }
    }
}

/// Polls chunk meshing tasks and adds the MeshComponent when ready.
pub fn poll_chunk_meshing_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkMeshingTaskComponent)>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
) {
    for (entity, mut meshing_task_component) in tasks_query.iter_mut() {
        let coord = meshing_task_component.coord;

        // check for cancellation from loader to avoid polling a to-be-killed entity
        // which could lead to race conditions.
        if !chunk_manager.meshing_chunks.contains_key(&coord) {
            debug!(
                target: "chunk_loading",
                "Chunk meshing task for {} was cancelled before completion.",
                coord
            );
            commands
                .entity(entity)
                .remove::<ChunkMeshingTaskComponent>();
            continue;
        }

        // poll mesh tasks
        if let Some(mesh_component_option) = now_or_never(&mut meshing_task_component.task) {
            debug!(target : "chunk_loading","Chunk meshing finished for {:?}", coord);

            // check if the meshing task actually produced a mesh
            if let Some(mesh_component) = mesh_component_option {
                // add mesh component and mark as loaded
                commands.entity(entity).insert(mesh_component);
                debug!(target: "chunk_loading","Chunk at {:?} is now fully loaded.", coord);
            } else {
                debug!(target: "chunk_loading", "Chunk at {:?} is empty, no mesh component added.", coord);
            }

            // mark as loaded and remove the task
            chunk_manager.mark_as_loaded(coord, entity);
            commands
                .entity(entity)
                .remove::<ChunkMeshingTaskComponent>();
        }
    }
}
