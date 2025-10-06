use crate::ecs_core::state_machine::resources::{CurrentState, NextState};
use crate::ecs_core::state_machine::State;
use crate::prelude::*;
use crate::render_world::extract::utils::run_extract_schedule::GameWorld;
use bevy_ecs::prelude::*;

/// Detects a state change in the game_world and queues a
/// corresponding state change in the render_world.
pub fn extract_state_system<T: State>(
    // Input
    main_world: Res<GameWorld>,
    render_world_state: Res<CurrentState<T>>,

    // Output next state
    mut next_state: ResMut<NextState<T>>,
) {
    let game_world_state = main_world.val.get_resource::<CurrentState<T>>().unwrap();

    // If the game_world has a state that the render_world doesn't have yet...
    if game_world_state.val != render_world_state.val {
        debug!(
            target: "state_machine",
            "Render world extracted a state change: {:?} -> {:?}",
            render_world_state.val, game_world_state.val
        );
        next_state.val = Some(game_world_state.val.clone());
    }
}
