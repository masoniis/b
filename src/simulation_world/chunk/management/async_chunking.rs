use crate::prelude::*;
use crate::simulation_world::chunk::core::{ActiveBiomeGenerator, GeneratedChunkComponentBundle};
use crate::simulation_world::chunk::{ChunkCoord, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::simulation_world::{
    asset_management::{texture_map_registry::TextureMapResource, AssetStorageResource, MeshAsset},
    block::property_registry::BlockRegistryResource,
    chunk::{
        chunk_meshing::build_chunk_mesh, load_manager::ChunkLoadManager, load_manager::ChunkState,
        ActiveChunkGenerator, ChunkBlocksComponent, MeshComponent, TransformComponent,
    },
};
use bevy_ecs::prelude::*;
use bevy_tasks::{futures::now_or_never, AsyncComputeTaskPool, Task};

/// Marks a chunk loading task in the simulation world that returns nothing.
#[derive(Component)]
pub struct ChunkGenerationTaskComponent {
    pub task: Task<GeneratedChunkComponentBundle>,
}

/// Marks a chunk meshing task in the simulation world that returns a MeshComponent.
#[derive(Component)]
pub struct ChunkMeshingTaskComponent {
    pub task: Task<Option<MeshComponent>>,
}

#[derive(Component)]
pub struct NeedsMeshing;

#[derive(Component)]
pub struct NeedsGenerating;

const MAX_GENERATION_STARTS_PER_FRAME: usize = 8;
const MAX_MESHING_STARTS_PER_FRAME: usize = 8;

/// Queries for entities needing generation and starts a limited number per frame.
#[instrument(skip_all)]
pub fn start_pending_generation_tasks_system(
    // Input
    mut pending_chunks_query: Query<
        (Entity, &NeedsGenerating, &ChunkCoord),
        Without<ChunkGenerationTaskComponent>,
    >,

    // Output/Resources
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
    block_registry: Res<BlockRegistryResource>,
    b_generator: Res<ActiveBiomeGenerator>,
    c_generator: Res<ActiveChunkGenerator>,

    // Local counter for throttling
    mut generation_tasks_started_this_frame: Local<usize>,
) {
    *generation_tasks_started_this_frame = 0;

    let task_pool = AsyncComputeTaskPool::get();

    for (entity, _, coord) in pending_chunks_query.iter_mut() {
        if *generation_tasks_started_this_frame >= MAX_GENERATION_STARTS_PER_FRAME {
            break;
        }

        // check for cancellation
        match chunk_manager.get_state(coord.pos) {
            Some(ChunkState::NeedsGenerating(state_entity)) if state_entity == entity => {
                // state is correct, proceed to start generation
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Entity {:?} NeedsGenerating for chunk {} found, but manager state ({:?}) doesn't match NeedsGenerating({:?}). Assuming cancelled/stale.",
                    entity, coord, chunk_manager.get_state(coord.pos), entity
                );
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
        let gen_clone = c_generator.0.clone();
        let bgen_clone = b_generator.0.clone();
        let coord_clone = coord.clone();
        let task = task_pool.spawn(async move {
            let biome_map = bgen_clone.generate_biome_map(coord_clone.pos);

            let tgen = gen_clone.generate_terrain_chunk(coord_clone.pos, &biome_map, &blocks_clone);

            GeneratedChunkComponentBundle {
                biome_map: biome_map,
                chunk_blocks: tgen.chunk_blocks,
                surface_heightmap: tgen.surface_heightmap,
                world_surface_heightmap: tgen.world_surface_heightmap,
            }
        });

        commands
            .entity(entity)
            .insert(ChunkGenerationTaskComponent { task })
            .remove::<NeedsGenerating>();

        chunk_manager.mark_as_generating(coord.pos, entity);
    }
}

/// Polls chunk generation tasks, adds generated components, and marks chunks as NeedsMeshing.
pub fn poll_chunk_generation_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkGenerationTaskComponent, &ChunkCoord)>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
) {
    for (entity, mut generation_task_component, coord) in tasks_query.iter_mut() {
        // check for cancellation using the manager state
        match chunk_manager.get_state(coord.pos) {
            Some(ChunkState::Generating(gen_entity)) if gen_entity == entity => {
                // state is correct, proceed
            }
            _ => {
                debug!(
                    target : "chunk_loading",
                    "Chunk generation task for {} found but manager state is not Generating({:?}). Assuming cancelled.",
                    coord, entity
                );
                continue;
            }
        }

        // poll the generation task
        if let Some(gen_bundle) = now_or_never(&mut generation_task_component.task) {
            debug!(
                target: "chunk_loading",
                "Chunk generation finished for {}. Marking as NeedsMeshing.",
                coord
            );

            commands
                .entity(entity)
                .insert((
                    gen_bundle.chunk_blocks,
                    gen_bundle.biome_map,
                    TransformComponent {
                        position: Vec3::new(
                            (coord.x * CHUNK_WIDTH as i32) as f32,
                            (coord.y * CHUNK_HEIGHT as i32) as f32,
                            (coord.z * CHUNK_DEPTH as i32) as f32,
                        ),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                    NeedsMeshing,
                ))
                .remove::<ChunkGenerationTaskComponent>();

            chunk_manager.mark_as_needs_meshing(coord.pos, entity);
        }
    }
}

/// Queries for chunks needing meshing and starts a limited number of tasks per frame.
#[instrument(skip_all)]
pub fn start_pending_meshing_tasks_system(
    // Input
    mut pending_chunks_query: Query<
        (
            Entity,
            &ChunkBlocksComponent,
            &ChunkCoord,
            &TransformComponent,
        ),
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

    for (entity, chunk_comp, chunk_coord, _transform_comp) in pending_chunks_query.iter_mut() {
        if *meshing_tasks_started_this_frame >= MAX_MESHING_STARTS_PER_FRAME {
            break;
        }

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

        *meshing_tasks_started_this_frame += 1;

        debug!(target: "chunk_loading", "Starting meshing task for {} ({} this frame).", chunk_coord.pos, *meshing_tasks_started_this_frame);

        // spawn the mesh task with resources
        let texture_map_clone = texture_map.clone();
        let block_registry_clone = block_registry.clone();
        let mesh_assets_clone = mesh_assets.clone();
        let chunk_component_for_task = chunk_comp.clone();
        let c = chunk_coord.clone();

        let task_pool = AsyncComputeTaskPool::get();
        let meshing_task_handle: Task<Option<MeshComponent>> = task_pool.spawn(async move {
            let (vertices, indices) = build_chunk_mesh(
                &chunk_component_for_task,
                &texture_map_clone,
                &block_registry_clone,
            );

            if !vertices.is_empty() {
                let mesh_asset = MeshAsset {
                    name: format!("chunk_{}_{}_{}", c.pos.x, c.pos.y, c.pos.z),
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
            })
            .remove::<NeedsMeshing>();

        chunk_manager.mark_as_meshing(chunk_coord.pos, entity);
    }
}

/// Polls chunk meshing tasks and adds the MeshComponent when ready.
pub fn poll_chunk_meshing_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkMeshingTaskComponent, &ChunkCoord)>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
) {
    for (entity, mut meshing_task_component, coord) in tasks_query.iter_mut() {
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
            chunk_manager.mark_as_loaded(coord.pos, entity);

            // remove the completed task component
            commands
                .entity(entity)
                .remove::<ChunkMeshingTaskComponent>();
        }
    }
}
