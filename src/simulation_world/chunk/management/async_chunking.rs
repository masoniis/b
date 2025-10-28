use crate::prelude::*;
use crate::simulation_world::biome::BiomeRegistryResource;
use crate::simulation_world::chunk::{ChunkCoord, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::simulation_world::generation::core::{
    ActiveBiomeGenerator, GeneratedChunkComponentBundle,
};
use crate::simulation_world::generation::{ActiveTerrainGenerator, ClimateNoiseGenerator};
use crate::simulation_world::{
    asset_management::{texture_map_registry::TextureMapResource, AssetStorageResource, MeshAsset},
    block::BlockRegistryResource,
    chunk::{
        chunk_meshing::build_chunk_mesh, load_manager::ChunkLoadManager, load_manager::ChunkState,
        ChunkBlocksComponent, MeshComponent, TransformComponent,
    },
};
use bevy_ecs::prelude::*;
use crossbeam::channel::{unbounded, Receiver, TryRecvError};

/// Marks a chunk loading task in the simulation world that returns nothing.
#[derive(Component)]
pub struct ChunkGenerationTaskComponent {
    pub receiver: Receiver<GeneratedChunkComponentBundle>,
}

/// Marks a chunk meshing task in the simulation world that returns a MeshComponent.
#[derive(Component)]
pub struct ChunkMeshingTaskComponent {
    pub receiver: Receiver<Option<MeshComponent>>,
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
    biome_registry: Res<BiomeRegistryResource>,
    b_generator: Res<ActiveBiomeGenerator>,
    c_generator: Res<ActiveTerrainGenerator>,
    climate_noise: Res<ClimateNoiseGenerator>,

    // Local counter for throttling
    mut generation_tasks_started_this_frame: Local<usize>,
) {
    *generation_tasks_started_this_frame = 0;

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
        let biomes_clone = biome_registry.clone();
        let gen_clone = c_generator.0.clone();
        let bgen_clone = b_generator.0.clone();
        let coord_clone = coord.clone();
        let climate_noise_clone = climate_noise.clone();

        let (sender, receiver) = unbounded();
        rayon::spawn(move || {
            let (biome_map, climate_map) = bgen_clone
                .generate_biome_chunk(&coord_clone, &climate_noise_clone, &biomes_clone)
                .as_tuple();

            let tgen = gen_clone.generate_terrain_chunk(
                coord_clone.pos,
                &biome_map,
                &climate_map,
                &blocks_clone,
                &biomes_clone,
            );

            trace!(
                target: "chunk_loading",
                "Finished generation for chunk {}.",
                coord_clone
            );

            let bundle = GeneratedChunkComponentBundle {
                biome_map: biome_map,
                chunk_blocks: tgen.chunk_blocks,
                ocean_floor_hmap: tgen.surface_heightmap,
                world_surface_hmap: tgen.world_surface_heightmap,
            };
            let _ = sender.send(bundle);
        });

        trace!(
            target: "chunk_loading",
            "Spawned generation task for chunk {}.",
            coord
        );

        commands
            .entity(entity)
            .insert(ChunkGenerationTaskComponent { receiver })
            .remove::<NeedsGenerating>();

        chunk_manager.mark_as_generating(coord.pos, entity);
    }
}

/// Polls chunk generation tasks, adds generated components, and marks chunks as NeedsMeshing.
#[instrument(skip_all)]
pub fn poll_chunk_generation_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkGenerationTaskComponent, &ChunkCoord)>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadManager>,
) {
    for (entity, generation_task_component, coord) in tasks_query.iter_mut() {
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
        match generation_task_component.receiver.try_recv() {
            Ok(gen_bundle) => {
                trace!(
                    target: "chunk_loading",
                    "Chunk generation finished for {}. Marking as NeedsMeshing.",
                    coord
                );

                commands
                    .entity(entity)
                    .insert((gen_bundle.chunk_blocks, gen_bundle.biome_map, NeedsMeshing))
                    .remove::<ChunkGenerationTaskComponent>();

                chunk_manager.mark_as_needs_meshing(coord.pos, entity);
            }
            Err(TryRecvError::Empty) => {
                // Task still running
            }
            Err(TryRecvError::Disconnected) => {
                warn!(
                    target: "chunk_loading",
                    "Chunk generation task for {} failed (channel disconnected). Despawning entity.",
                    coord
                );
                commands.entity(entity).despawn();
            }
        }
    }
}

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
        (With<NeedsMeshing>, Without<ChunkMeshingTaskComponent>),
    >,
    all_generated_chunks: Query<&ChunkBlocksComponent>, // for finding neighbors

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

    'chunk_loop: for (entity, chunk_comp, chunk_coord) in pending_chunks_query.iter_mut() {
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
            None => continue 'chunk_loop,
        };
        let left = match get_neighbor(IVec3::new(-1, 0, 0)) {
            Some(data) => data,
            None => continue 'chunk_loop,
        };
        let top = match get_neighbor(IVec3::new(0, 1, 0)) {
            Some(data) => data,
            None => continue 'chunk_loop,
        };
        let bottom = match get_neighbor(IVec3::new(0, -1, 0)) {
            Some(data) => data,
            None => continue 'chunk_loop,
        };
        let front = match get_neighbor(IVec3::new(0, 0, 1)) {
            Some(data) => data,
            None => continue 'chunk_loop,
        };
        let back = match get_neighbor(IVec3::new(0, 0, -1)) {
            Some(data) => data,
            None => continue 'chunk_loop,
        };

        let neighbor_data_for_task = ChunkNeighborData {
            right,
            left,
            top,
            bottom,
            front,
            back,
        };

        trace!(target: "chunk_loading", "Starting meshing task for {} ({} this frame).", chunk_coord.pos, *meshing_tasks_started_this_frame);

        // INFO: -----------------------------
        //         Spawn the mesh task
        // -----------------------------------

        *meshing_tasks_started_this_frame += 1;
        let texture_map_clone = texture_map.clone();
        let block_registry_clone = block_registry.clone();
        let mesh_assets_clone = mesh_assets.clone();
        let chunk_component_for_task = chunk_comp.clone();
        let c = chunk_coord.clone();

        let (sender, receiver) = unbounded();
        rayon::spawn(move || {
            let (vertices, indices) = build_chunk_mesh(
                &chunk_component_for_task,
                &neighbor_data_for_task,
                &texture_map_clone,
                &block_registry_clone,
            );

            let result = if !vertices.is_empty() {
                let mesh_asset = MeshAsset {
                    name: format!("chunk_{}_{}_{}", c.pos.x, c.pos.y, c.pos.z),
                    vertices,
                    indices,
                };
                let mesh_handle = mesh_assets_clone.add(mesh_asset);
                Some(MeshComponent::new(mesh_handle))
            } else {
                None
            };
            let _ = sender.send(result);
        });

        // update entity and manager
        commands
            .entity(entity)
            .insert(ChunkMeshingTaskComponent { receiver })
            .remove::<NeedsMeshing>();

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
    mut chunk_manager: ResMut<ChunkLoadManager>,
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
            Ok(mesh_component_option) => {
                trace!(target : "chunk_loading","Chunk meshing finished for {:?}", coord);

                // add MeshComponent if it exists
                if let Some(mesh_component) = mesh_component_option {
                    commands.entity(entity).insert((
                        mesh_component,
                        TransformComponent {
                            position: Vec3::new(
                                (coord.x * CHUNK_WIDTH as i32) as f32,
                                (coord.y * CHUNK_HEIGHT as i32) as f32,
                                (coord.z * CHUNK_DEPTH as i32) as f32,
                            ),
                            rotation: Quat::IDENTITY,
                            scale: Vec3::ONE,
                        },
                    ));
                    trace!(target: "chunk_loading","Chunk at {:?} is now fully loaded.", coord);
                } else {
                    trace!(target: "chunk_loading", "Chunk at {:?} is empty, no mesh component added.", coord);
                    // the chunk was empty so no mesh was added
                }
                chunk_manager.mark_as_loaded(coord.pos, entity);

                // remove the completed task component
                commands
                    .entity(entity)
                    .remove::<ChunkMeshingTaskComponent>();
            }
            Err(TryRecvError::Empty) => {
                // Task still running
            }
            Err(TryRecvError::Disconnected) => {
                warn!(
                    target: "chunk_loading",
                    "Chunk meshing task for {} failed (channel disconnected). Despawning entity.",
                    coord
                );
                commands.entity(entity).despawn();
            }
        }
    }
}
