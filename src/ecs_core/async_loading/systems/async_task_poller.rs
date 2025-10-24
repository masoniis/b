use crate::{
    ecs_core::async_loading::{
        load_tracking::LoadingTaskTracker, loading_task::SimulationWorldLoadingTaskComponent,
        LoadingTracker, RenderWorldLoadingTaskComponent,
    },
    prelude::*,
};
use bevy_ecs::prelude::*;
use bevy_tasks::futures::now_or_never;
use futures_lite::future;

/// Polls simulation-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_simulation_loading_tasks(
    // Input
    mut tasks: Query<(Entity, &mut SimulationWorldLoadingTaskComponent)>,

    // Output (updated states)
    mut task_tracker: ResMut<LoadingTaskTracker>,
    mut commands: Commands,
    loading_tracker: Res<LoadingTracker>,
) {
    if !task_tracker.has_spawned_tasks() {
        return; // no tasks spawned yet
    }

    for (entity, mut task_component) in tasks.iter_mut() {
        if let Some(callback) = now_or_never(&mut task_component.task) {
            info!("[POLL] Task completed! Executing callback...");
            callback(&mut commands);
            commands.entity(entity).despawn();

            task_tracker.register_completion();
        }
    }

    if task_tracker.all_complete() && !loading_tracker.is_simulation_ready() {
        debug!(
            target: "async_tasks",
            "[POLL] All {} spawned tasks are complete. Marking simulation ready.",
            task_tracker.spawned
        );
        loading_tracker.set_simulation_ready(true);
    }
}

/// Polls render-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_render_loading_tasks(
    // Input
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut RenderWorldLoadingTaskComponent)>,

    // Output (update the shared tracker)
    loading_tracker: Res<LoadingTracker>,
) {
    let mut remaining_tasks = 0;

    for (entity, mut task_component) in tasks.iter_mut() {
        if let Some(callback) = future::block_on(future::poll_once(&mut task_component.task)) {
            debug!(target: "async_tasks", "Render task completed, executing callback...");
            callback(&mut commands);
            commands.entity(entity).despawn();
        } else {
            remaining_tasks += 1;
        }
    }

    if remaining_tasks == 0 && !loading_tracker.is_renderer_ready() {
        info!("Render world is ready.");
        loading_tracker.set_renderer_ready(true);
    }
}
