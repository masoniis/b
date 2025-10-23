use crate::{
    ecs_core::async_loading::{
        loading_task::{
            AsyncTask, RenderWorldLoadingTaskComponent, SimulationWorldLoadingTaskComponent,
        },
        LoadingTracker,
    },
    prelude::*,
};
use bevy_ecs::prelude::*;

/// Polls simulation-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_simulation_loading_tasks(
    mut commands: Commands,
    mut tasks_query: Query<(Entity, &mut SimulationWorldLoadingTaskComponent)>,
    loading_tracker: Res<LoadingTracker>,
) {
    let mut remaining_tasks = 0;

    for (entity, mut task) in tasks_query.iter_mut() {
        if let Some(callback) = task.poll_result() {
            // Task completed - execute the callback immediately
            callback(&mut commands);

            // Despawn the task entity
            commands.entity(entity).despawn();
        } else {
            // Task still running
            remaining_tasks += 1;
        }
    }

    if remaining_tasks == 0 && !loading_tracker.is_simulation_ready() {
        info!("Simulation world is ready.");
        loading_tracker.set_simulation_ready(true);
    }
}

/// Polls render-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_render_loading_tasks(
    mut commands: Commands,
    mut tasks_query: Query<(Entity, &mut RenderWorldLoadingTaskComponent)>,
    loading_tracker: Res<LoadingTracker>,
) {
    let mut remaining_tasks = 0;

    for (entity, mut task) in tasks_query.iter_mut() {
        if let Some(callback) = task.poll_result() {
            // Task completed - execute the callback immediately
            callback(&mut commands);

            // Despawn the task entity
            commands.entity(entity).despawn();
        } else {
            // Task still running
            remaining_tasks += 1;
        }
    }

    if remaining_tasks == 0 && !loading_tracker.is_renderer_ready() {
        info!("Render world is ready.");
        loading_tracker.set_renderer_ready(true);
    }
}
