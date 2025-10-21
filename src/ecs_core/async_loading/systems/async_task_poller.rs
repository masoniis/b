use crate::{
    ecs_core::async_loading::{
        loading_task::{
            LoadingTask, RenderWorldLoadingTaskComponent, SimulationWorldLoadingTaskComponent,
        },
        LoadingTracker,
    },
    prelude::*,
};
use bevy_ecs::{component::Mutable, prelude::*};

/// Polls simulation-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_simulation_loading_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SimulationWorldLoadingTaskComponent)>,
    loading_tracker: Res<LoadingTracker>,
) {
    let remaining = poll_loading_tasks(&mut commands, &mut tasks);
    if remaining == 0 && !loading_tracker.is_simulation_ready() {
        info!("Simulation world is ready.");
        loading_tracker.set_simulation_ready(true);
    }
}

/// Polls render-specific tasks and updates the shared `LoadingTracker`.
#[instrument(skip_all)]
pub fn poll_render_loading_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut RenderWorldLoadingTaskComponent)>,
    loading_tracker: Res<LoadingTracker>,
) {
    let remaining = poll_loading_tasks(&mut commands, &mut tasks);
    if remaining == 0 && !loading_tracker.is_renderer_ready() {
        info!("Render world is ready.");
        loading_tracker.set_renderer_ready(true);
    }
}

/// A generic function that polls all tasks of a given type.
///
/// Executes callbacks for completed tasks, despawns their entities,
/// and returns the number of tasks still running.
fn poll_loading_tasks<T>(
    commands: &mut Commands,
    tasks_query: &mut Query<(Entity, &mut T)>,
) -> usize
where
    T: LoadingTask + Component<Mutability = Mutable>,
{
    let mut remaining_tasks = 0;

    for (entity, mut task) in tasks_query.iter_mut() {
        if let Some(callback) = task.poll_result() {
            // Task completed - execute the callback
            callback(commands);
            // Despawn the task entity
            commands.entity(entity).despawn();
        } else {
            // Task still running
            remaining_tasks += 1;
        }
    }

    remaining_tasks
}
