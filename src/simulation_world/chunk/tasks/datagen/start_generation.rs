use crate::prelude::*;
use crate::simulation_world::chunk::{ChunkGenerationTaskComponent, ChunkState, NeedsGenerating};
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    block::BlockRegistryResource,
    chunk::{ChunkCoord, ChunkStateManager},
    generation::{
        core::{ActiveBiomeGenerator, GeneratedChunkComponentBundle},
        ActiveTerrainGenerator, ClimateNoiseGenerator,
    },
};
use bevy_ecs::prelude::*;
use crossbeam::channel::unbounded;

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
    mut chunk_manager: ResMut<ChunkStateManager>,
    block_registry: Res<BlockRegistryResource>,
    biome_registry: Res<BiomeRegistryResource>,
    biome_generator: Res<ActiveBiomeGenerator>,
    terrain_generator: Res<ActiveTerrainGenerator>,
    climate_noise: Res<ClimateNoiseGenerator>,
) {
    for (entity, _, coord) in pending_chunks_query.iter_mut() {
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

        // check if the chunk is empty according to the terrain generator
        if terrain_generator.0.is_chunk_empty(coord.pos) {
            trace!(
                target: "chunk_loading",
                "Chunk {} is empty according to terrain generator. Skipping generation.",
                coord
            );
            commands.entity(entity).despawn();
            chunk_manager.mark_as_loaded_but_empty(coord.pos);
            continue;
        }

        // start the generation thread task if not
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
