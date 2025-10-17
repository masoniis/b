use crate::{
    ecs_core::async_loading::{
        loading_task::{LoadingTask, SimulationWorldLoadingTaskComponent},
        LoadingTracker,
    },
    prelude::*,
};
use bevy_ecs::prelude::*;

/// Polls simulation-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_simulation_loading_tasks(
    // Input
    mut commands: Commands,
    tasks: Query<(Entity, &SimulationWorldLoadingTaskComponent)>,

    // Output (update the shared tracker)
    loading_tracker: Res<LoadingTracker>,
) {
    let remaining = poll_loading_tasks(&mut commands, &tasks);
    if remaining == 0 && !loading_tracker.is_simulation_ready() {
        info!("Simulation world is ready.");
        loading_tracker.set_simulation_ready(true);
    }
}

/// Polls render-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_render_loading_tasks(
    // Input
    mut commands: Commands,
    tasks: Query<(Entity, &SimulationWorldLoadingTaskComponent)>,

    // Output (update the shared tracker)
    loading_tracker: Res<LoadingTracker>,
) {
    let remaining = poll_loading_tasks(&mut commands, &tasks);
    if remaining == 0 && !loading_tracker.is_renderer_ready() {
        info!("Render world is ready.");
        loading_tracker.set_renderer_ready(true);
    }
}

/// A generic function that polls all tasks of a given type.
///
/// Despawns completed tasks and returns the number of tasks still running.
fn poll_loading_tasks<T: LoadingTask>(
    commands: &mut Commands,
    tasks_query: &Query<(Entity, &T)>,
) -> usize {
    let mut remaining_tasks = 0;
    for (entity, task) in tasks_query.iter() {
        if task.is_finished() {
            commands.entity(entity).despawn();
        } else {
            remaining_tasks += 1;
        }
    }

    remaining_tasks
}
