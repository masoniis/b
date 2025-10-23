use crate::ecs_core::async_loading::loading_task::TokioTask;
use crate::simulation_world::chunk::load_manager::ChunkLoadManager;
use crate::simulation_world::chunk::MeshComponent;
use crate::{
    ecs_core::async_loading::loading_task::AsyncTask, simulation_world::chunk::GeneratedChunkData,
};
use bevy_ecs::prelude::Component;
use bevy_ecs::prelude::*;
use glam::IVec3;

/// Marks a chunk loading task in the simulation world that returns nothing.
#[derive(Component)]
pub struct ChunkGenerationTaskComponent {
    pub task: TokioTask<GeneratedChunkData>,
    pub coord: IVec3,
}

impl AsyncTask<GeneratedChunkData> for ChunkGenerationTaskComponent {
    fn poll_result(&mut self) -> Option<GeneratedChunkData> {
        self.task.poll("Chunk generation task failed.")
    }
}

/// Marks a chunk meshing task in the simulation world that returns a MeshComponent.
#[derive(Component)]
pub struct ChunkMeshingTaskComponent {
    pub task: TokioTask<MeshComponent>,
}

impl AsyncTask<MeshComponent> for ChunkMeshingTaskComponent {
    fn poll_result(&mut self) -> Option<MeshComponent> {
        self.task.poll("Chunk meshing task failed.")
    }
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
        if let Some(data) = task_component.poll_result() {
            // spawn in chunk data and mark as loaded
            let ent = commands.spawn((data.chunk, data.transform)).id();
            chunk_manager.mark_as_loaded(task_component.coord, ent);

            // despawn task
            commands.entity(entity).despawn();
        }
    }
}
