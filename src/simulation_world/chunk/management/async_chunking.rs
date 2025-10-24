use crate::prelude::*;
use crate::simulation_world::chunk::load_manager::ChunkLoadManager;
use crate::simulation_world::chunk::GeneratedChunkData;
use crate::simulation_world::chunk::MeshComponent;
use bevy_ecs::prelude::Component;
use bevy_ecs::prelude::*;
use bevy_tasks::futures::now_or_never;
use bevy_tasks::Task;
use glam::IVec3;

/// Marks a chunk loading task in the simulation world that returns nothing.
#[derive(Component)]
pub struct ChunkGenerationTaskComponent {
    pub task: Task<GeneratedChunkData>,
    pub coord: IVec3,
}

/// Marks a chunk meshing task in the simulation world that returns a MeshComponent.
#[derive(Component)]
pub struct ChunkMeshingTaskComponent {
    pub task: Task<MeshComponent>,
    pub coord: IVec3,
}

/// Polls all chunk generation tasks and spawns in the resulting chunk data when ready.
pub fn poll_chunk_generation_tasks(
    // Input
    mut tasks_query: Query<(Entity, &mut ChunkGenerationTaskComponent)>,

    // Output
    mut commands: Commands,                      // spawn in entity
    mut chunk_manager: ResMut<ChunkLoadManager>, // marking chunk loaded
) {
    for (entity, mut task_component) in tasks_query.iter_mut() {
        let coord = task_component.coord;

        // check for cancellation
        if !chunk_manager.generating_chunks.contains_key(&coord) {
            info!(
                "Chunk generation task for chunk {} was cancelled before completion.",
                coord
            );
            commands.entity(entity).despawn();
            continue;
        }

        if let Some(data) = now_or_never(&mut task_component.task) {
            // spawn in chunk entity
            let ent = commands
                .spawn((data.chunk_component, data.transform_component))
                .id();

            commands
                .entity(entity)
                .remove::<ChunkGenerationTaskComponent>();
            // TODO: loaded should only occur after meshing

            chunk_manager.mark_as_loaded(coord, ent);
        }
    }
}
