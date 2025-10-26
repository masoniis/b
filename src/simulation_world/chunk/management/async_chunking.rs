use crate::prelude::*;
use crate::simulation_world::{
    asset_management::{texture_map_registry::TextureMapResource, AssetStorageResource, MeshAsset},
    block::property_registry::BlockRegistryResource,
    chunk::{
        chunk_meshing::build_chunk_mesh, load_manager::ChunkLoadManager, load_manager::ChunkState,
        ActiveChunkGenerator, ChunkComponent, GeneratedChunkComponents, MeshComponent,
        TransformComponent,
    },
};
use bevy_ecs::prelude::*;
use bevy_tasks::{futures::now_or_never, AsyncComputeTaskPool, Task};
use glam::IVec3;

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

#[derive(Component)]
pub struct NeedsMeshing;

#[derive(Component)]
pub struct NeedsGenerating {
    pub coord: IVec3,
}

const MAX_GENERATION_STARTS_PER_FRAME: usize = 8;
const MAX_MESHING_STARTS_PER_FRAME: usize = 8;

/// Queries for entities needing generation and starts a limited number per frame.
#[instrument(skip_all)]
pub fn start_pending_generation_tasks_system(
    // Input
    mut pending_chunks_query: Query<
        (Entity, &NeedsGenerating),
        Without<ChunkGenerationTaskComponent>,
    >,

    // Output/Resources
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
    block_registry: Res<BlockRegistryResource>,
    generator: Res<ActiveChunkGenerator>,

    // Local counter for throttling
    mut generation_tasks_started_this_frame: Local<usize>,
) {
    *generation_tasks_started_this_frame = 0;

    let task_pool = AsyncComputeTaskPool::get();

    for (entity, needs_gen) in pending_chunks_query.iter_mut() {
        if *generation_tasks_started_this_frame >= MAX_GENERATION_STARTS_PER_FRAME {
            break;
        }

        // check for cancellation
        let coord = needs_gen.coord;
        match chunk_manager.get_state(coord) {
            Some(ChunkState::NeedsGenerating(state_entity)) if state_entity == entity => {
                // state is correct, proceed to start generation
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Entity {:?} NeedsGenerating for chunk {} found, but manager state ({:?}) doesn't match NeedsGenerating({:?}). Assuming cancelled/stale.",
                    entity, coord, chunk_manager.get_state(coord), entity
                );
                // commands.entity(entity).remove::<NeedsGenerating>();
                continue;
            }
        }

        *generation_tasks_started_this_frame += 1;

        debug!(
            target: "chunk_loading",
            "Starting generation task for {} ({} this frame).",
            coord, *generation_tasks_started_this_frame
        );

        // spawn in the task with resources needed
        let blocks_clone = block_registry.clone();
        let gen_clone = generator.0.clone();
        let task = task_pool.spawn(async move { gen_clone.generate_chunk(coord, &blocks_clone) });

        commands
            .entity(entity)
            .insert(ChunkGenerationTaskComponent { task, coord })
            .remove::<NeedsGenerating>();

        chunk_manager.mark_as_generating(coord, entity);
    }
}

/// Polls chunk generation tasks, adds generated components, and marks chunks as NeedsMeshing.
pub fn poll_chunk_generation_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkGenerationTaskComponent)>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
) {
    for (entity, mut generation_task_component) in tasks_query.iter_mut() {
        let coord = generation_task_component.coord;

        // check for cancellation using the manager state
        match chunk_manager.get_state(coord) {
            Some(ChunkState::Generating(gen_entity)) if gen_entity == entity => {
                // state is correct, proceed
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Chunk generation task for {} found but manager state is not Generating({:?}). Assuming cancelled.",
                    coord, entity
                );
                // commands
                //     .entity(entity)
                //     .remove::<ChunkGenerationTaskComponent>();
                continue;
            }
        }

        // poll the generation task
        if let Some(generated_data) = now_or_never(&mut generation_task_component.task) {
            debug!(
                target: "chunk_loading",
                "Chunk generation finished for {}. Marking as NeedsMeshing.",
                coord
            );

            commands
                .entity(entity)
                .insert((
                    generated_data.chunk_component,
                    generated_data.transform_component,
                    NeedsMeshing,
                ))
                .remove::<ChunkGenerationTaskComponent>();

            chunk_manager.mark_as_needs_meshing(coord, entity);
        }
    }
}

/// Queries for chunks needing meshing and starts a limited number of tasks per frame.
#[instrument(skip_all)]
pub fn start_pending_meshing_tasks_system(
    // Input
    mut pending_chunks_query: Query<
        (Entity, &ChunkComponent, &TransformComponent),
        (With<NeedsMeshing>, Without<ChunkMeshingTaskComponent>),
    >,

    // Resources needed to start meshing
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
    texture_map: Res<TextureMapResource>,
    block_registry: Res<BlockRegistryResource>,
    mesh_assets: Res<AssetStorageResource<MeshAsset>>,

    // Local counter for throttling
    mut meshing_tasks_started_this_frame: Local<usize>,
) {
    *meshing_tasks_started_this_frame = 0;

    for (entity, chunk_comp, _transform_comp) in pending_chunks_query.iter_mut() {
        if *meshing_tasks_started_this_frame >= MAX_MESHING_STARTS_PER_FRAME {
            break;
        }

        // check for cancellation
        let coord = chunk_comp.coord;
        match chunk_manager.get_state(coord) {
            Some(ChunkState::NeedsMeshing(state_entity)) if state_entity == entity => {
                // state is correct, proceed to start meshing
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Chunk {} marked NeedsMeshing but manager state is not NeedsMeshing({:?}). Assuming cancelled.",
                    coord, entity
                );
                // entity will be despawned next frame (or already)
                // commands.entity(entity).remove::<NeedsMeshing>();
                continue;
            }
        }

        *meshing_tasks_started_this_frame += 1;

        debug!(target: "chunk_loading", "Starting meshing task for {} ({} this frame).", coord, *meshing_tasks_started_this_frame);

        // spawn the mesh task with resources
        let texture_map_clone = texture_map.clone();
        let block_registry_clone = block_registry.clone();
        let mesh_assets_clone = mesh_assets.clone();
        let chunk_component_for_task = chunk_comp.clone();

        let task_pool = AsyncComputeTaskPool::get();
        let meshing_task_handle: Task<Option<MeshComponent>> = task_pool.spawn(async move {
            let (vertices, indices) = build_chunk_mesh(
                &chunk_component_for_task,
                &texture_map_clone,
                &block_registry_clone,
            );

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

        // update entity and manager
        commands
            .entity(entity)
            .insert(ChunkMeshingTaskComponent {
                task: meshing_task_handle,
                coord,
            })
            .remove::<NeedsMeshing>();

        chunk_manager.mark_as_meshing(coord, entity);
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

        // check for cancellation
        match chunk_manager.get_state(coord) {
            Some(ChunkState::Meshing(state_entity)) if state_entity == entity => {
                // state is correct, proceed
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Chunk meshing task for {} found but manager state is not Meshing({:?}). Assuming cancelled.",
                    coord, entity
                );
                // commands
                //     .entity(entity)
                //     .remove::<ChunkMeshingTaskComponent>();
                continue;
            }
        }

        // poll mesh task
        if let Some(mesh_component_option) = now_or_never(&mut meshing_task_component.task) {
            debug!(target : "chunk_loading","Chunk meshing finished for {:?}", coord);

            // add MeshComponent if it exists
            if let Some(mesh_component) = mesh_component_option {
                commands.entity(entity).insert(mesh_component);
                debug!(target: "chunk_loading","Chunk at {:?} is now fully loaded.", coord);
            } else {
                debug!(target: "chunk_loading", "Chunk at {:?} is empty, no mesh component added.", coord);
                // the chunk was empty so no mesh was added
            }
            chunk_manager.mark_as_loaded(coord, entity);

            // remove the completed task component
            commands
                .entity(entity)
                .remove::<ChunkMeshingTaskComponent>();
        }
    }
}
