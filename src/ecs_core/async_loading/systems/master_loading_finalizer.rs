use crate::{
    ecs_core::{
        async_loading::{LoadingTracker, OnLoadComplete},
        state_machine::{resources::NextState, AppState},
    },
    prelude::*,
};
use bevy_ecs::prelude::*;

/// The master system that runs in the simulation world. It checks the shared
/// LoadingTracker and the OnLoadComplete resource to make the final decision
/// on when to transition the app's state.
pub fn master_finalize_loading_system(
    // Input
    loading_tracker: Res<LoadingTracker>,
    on_complete: Option<Res<OnLoadComplete<AppState>>>,

    // Output (set the next state)
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    // if we have both the tracker and the "what to do next" instruction
    if let Some(on_complete) = on_complete {
        if loading_tracker.is_all_ready() {
            info!(
                "All worlds are ready. Transitioning to {:?}.",
                on_complete.destination
            );

            // Set the next state within our own world.
            next_state.val = Some(on_complete.destination.clone());

            // Clean up the temporary resources for the next loading operation.
            commands.remove_resource::<OnLoadComplete<AppState>>();
        }
    }
}
