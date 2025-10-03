pub use crate::ecs_modules::{
    schedules::{OnEnter, OnExit},
    state_machine::resources::{CurrentState, NextState},
    state_machine::types::State,
};
use crate::prelude::*;
use bevy_ecs::prelude::*;

pub fn apply_state_transition_system<T: State>(world: &mut World) {
    // Because we are running schedules we need to extract next state first
    let next_state_opt = world
        .get_resource_mut::<NextState<T>>()
        .and_then(|mut next_state_res| next_state_res.val.take());

    // If a state transition was requested...
    if let Some(new_state) = next_state_opt {
        let old_state = world.resource::<CurrentState<T>>().val.clone();

        if old_state == new_state {
            return;
        }

        info!("\n\nState transition: {:?} -> {:?}\n", old_state, new_state);

        // INFO: Try-run the transition schedules
        if let Err(e) = world.try_run_schedule(OnExit(old_state.clone())) {
            warn!("Transition didn't run: {}", e);
        }

        // Update the CurrentState resource
        let mut current_state_res = world.resource_mut::<CurrentState<T>>();
        current_state_res.val = new_state.clone();

        if let Err(e) = world.try_run_schedule(OnEnter(new_state.clone())) {
            warn!("Transition didn't run: {}", e);
        }
    }
}
