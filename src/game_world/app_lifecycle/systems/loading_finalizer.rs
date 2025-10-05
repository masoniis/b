use crate::core::state_machine::resources::NextState;
use crate::game_world::app_lifecycle::{AppState, LoadingTaskComponent};
use crate::prelude::*;
use bevy_ecs::prelude::*;
use futures_lite::future;

/// This system should run every frame during the Loading state. It checks all
/// running `LoadingTask`s. If a task is complete, it is despawned. If no task
/// is left running, it makes the transition to AppState::Running.
pub fn finalize_loading_system(
    // World manipulation
    mut commands: Commands,

    // Input
    mut tasks: Query<(Entity, &mut LoadingTaskComponent)>,

    // Output
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Poll all async tasks to determine if any are still active
    let mut remaining_tasks = 0;
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(_) = future::block_on(future::poll_once(&mut task.handle)) {
            commands.entity(entity).despawn();
        } else {
            remaining_tasks += 1;
        }
    }

    // Transition to AppState::Running if no tasks remain
    if remaining_tasks == 0 {
        info!("All loading tasks complete. Transitioning to AppState::Running.");
        next_state.val = Some(AppState::Running);
    }
}
