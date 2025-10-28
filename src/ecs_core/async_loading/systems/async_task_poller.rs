use crate::{
    ecs_core::async_loading::{
        loading_task::SimulationWorldLoadingTaskComponent, LoadingTracker,
        RenderWorldLoadingTaskComponent,
    },
    prelude::*,
};
use bevy_ecs::prelude::*;
use crossbeam::channel::TryRecvError;

/// Polls simulation-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_simulation_loading_tasks(
    // Input
    mut tasks: Query<(Entity, &mut SimulationWorldLoadingTaskComponent)>,

    // Output (updated states)
    mut commands: Commands,
    loading_tracker: Res<LoadingTracker>,
) {
    // Local counter to track tasks that are still running this frame.
    // This correctly handles the case where 0 tasks were spawned.
    let mut remaining_tasks = 0;

    for (entity, task) in &mut tasks {
        match task.receiver.try_recv() {
            Ok(callback) => {
                callback(&mut commands);
                commands.entity(entity).despawn();
            }
            Err(TryRecvError::Empty) => {
                // Task is still working, count it.
                remaining_tasks += 1;
            }
            Err(TryRecvError::Disconnected) => {
                eprintln!("Task failed: Channel disconnected!");
                commands.entity(entity).despawn();
            }
        }
    }

    // Use the local counter to determine if all tasks are done.
    if remaining_tasks == 0 && !loading_tracker.is_simulation_ready() {
        debug!(
            target: "async_tasks",
            "[POLL] All tasks are complete. Marking simulation ready.",
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

    for (entity, task_component) in tasks.iter_mut() {
        // Updated to use the receiver pattern
        match task_component.receiver.try_recv() {
            Ok(callback) => {
                debug!(target: "async_tasks", "Render task completed, executing callback...");
                callback(&mut commands);
                commands.entity(entity).despawn();
            }
            Err(TryRecvError::Empty) => {
                remaining_tasks += 1; // task is still working
            }
            Err(TryRecvError::Disconnected) => {
                eprintln!("Task failed: Channel disconnected!");
                commands.entity(entity).despawn();
            }
        }
    }

    if remaining_tasks == 0 && !loading_tracker.is_renderer_ready() {
        info!("Render world is ready.");
        loading_tracker.set_renderer_ready(true);
    }
}
