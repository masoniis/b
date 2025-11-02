use crate::prelude::*;
use crate::simulation_world::camera::ActiveCamera;
use crate::simulation_world::chunk::manage_mesh_state::chunk_is_in_mesh_radius;
use crate::simulation_world::chunk::ChunkState;
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    block::BlockRegistryResource,
    chunk::{ChunkCoord, ChunkLoadingManager},
    generation::{
        core::{ActiveBiomeGenerator, GeneratedChunkComponentBundle},
        ActiveTerrainGenerator, ClimateNoiseGenerator,
    },
};
use bevy_ecs::prelude::*;
use crossbeam::channel::{unbounded, Receiver, TryRecvError};

/// Marks a chunk loading task in the simulation world that returns nothing.
#[derive(Component)]
pub struct ChunkGenerationTaskComponent {
    pub receiver: Receiver<GeneratedChunkComponentBundle>,
}

#[derive(Component)]
pub struct NeedsMeshing;

#[derive(Component)]
pub struct NeedsGenerating;

const MAX_GENERATION_STARTS_PER_FRAME: usize = 64;

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
    mut chunk_manager: ResMut<ChunkLoadingManager>,
    block_registry: Res<BlockRegistryResource>,
    biome_registry: Res<BiomeRegistryResource>,
    biome_generator: Res<ActiveBiomeGenerator>,
    terrain_generator: Res<ActiveTerrainGenerator>,
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
        let (sender, receiver) = unbounded();

        let blocks_clone = block_registry.clone();
        let biomes_clone = biome_registry.clone();
        let terrain_gen = terrain_generator.clone();
        let biome_gen = biome_generator.clone();
        let climate_noise_clone = climate_noise.clone();
        let coord_clone = coord.clone();

        rayon::spawn(move || {
            let (biome_map, climate_map) = biome_gen
                .generate_biome_chunk(&coord_clone, &climate_noise_clone, &biomes_clone)
                .as_tuple();

            let tgen = terrain_gen.generate_terrain_chunk(
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
                chunk_blocks: tgen.chunk_blocks,
                biome_map: biome_map,
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

/// Polls chunk generation tasks, adds generated components, and marks chunks as
/// `NeedsMeshing` (if in range) or `DataReady` (if out of range).
#[instrument(skip_all)]
pub fn poll_chunk_generation_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkGenerationTaskComponent, &ChunkCoord)>,
    active_camera: Res<ActiveCamera>, // to gauge if chunk is in meshing range
    camera_query: Query<&ChunkCoord>,

    // Output
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkLoadingManager>,
) {
    let camera_chunk_pos = camera_query.get(active_camera.0).map(|c| c.pos);

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
                commands
                    .entity(entity)
                    .remove::<ChunkGenerationTaskComponent>();
                continue;
            }
        }

        // poll the generation task
        match generation_task_component.receiver.try_recv() {
            Ok(gen_bundle) => {
                if let Some(chunk_blocks) = gen_bundle.chunk_blocks {
                    let mut is_in_mesh_radius = false;
                    if let Ok(cam_pos) = camera_chunk_pos {
                        is_in_mesh_radius = chunk_is_in_mesh_radius(cam_pos, coord.pos);
                    }

                    if is_in_mesh_radius {
                        trace!(
                            target: "chunk_loading",
                            "Chunk generation finished for {}. In range. Marking as NeedsMeshing.",
                            coord
                        );
                        commands
                            .entity(entity)
                            .insert((chunk_blocks, gen_bundle.biome_map, NeedsMeshing))
                            .remove::<ChunkGenerationTaskComponent>();
                        chunk_manager.mark_as_needs_meshing(coord.pos, entity);
                    } else {
                        trace!(
                            target: "chunk_loading",
                            "Chunk generation finished for {}. Out of range. Marking as DataReady.",
                            coord
                        );
                        commands
                            .entity(entity)
                            .insert((chunk_blocks, gen_bundle.biome_map))
                            .remove::<ChunkGenerationTaskComponent>();
                        chunk_manager.mark_as_data_ready(coord.pos, entity);
                    }
                } else {
                    trace!(
                        target: "chunk_loading",
                        "Chunk generation finished for {} but chunk is empty. Marking as Loaded(None).",
                        coord
                    );
                    commands.entity(entity).despawn();
                    chunk_manager.mark_as_loaded_but_empty(coord.pos);
                }
            }
            Err(TryRecvError::Empty) => {
                // task still running
            }
            Err(TryRecvError::Disconnected) => {
                warn!(
                    target: "chunk_loading",
                    "Chunk generation task for {} failed (channel disconnected). Despawning entity.",
                    coord
                );
                commands.entity(entity).despawn();
                chunk_manager.mark_as_unloaded(coord.pos);
            }
        }
    }
}
