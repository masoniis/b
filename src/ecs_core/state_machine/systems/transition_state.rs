use crate::ecs_core::state_machine::{
    resources::{CurrentState, NextState},
    State,
};
use crate::ecs_core::worlds::SimulationWorldMarker;
use crate::prelude::*;
pub use crate::simulation_world::schedules::{OnEnter, OnExit};
use bevy_ecs::prelude::*;

pub fn apply_state_transition_system<T: State>(world: &mut World) {
    // Because we are running schedules we need to extract next state first
    let next_state_opt = world
        .get_resource_mut::<NextState<T>>()
        .and_then(|mut next_state_res| next_state_res.val.take());

    // If a state transition was requested...
    if let Some(new_state) = next_state_opt {
        let is_simulation_world = world.get_resource::<SimulationWorldMarker>().is_some();

        let old_state = world.resource::<CurrentState<T>>().val.clone();

        if old_state == new_state {
            return;
        }

        if is_simulation_world {
            info!("\n\nState transition: {:?} -> {:?}\n", old_state, new_state);
        }

        // INFO: Try-run the transition schedules

        let curr_world = if is_simulation_world {
            "simulation"
        } else {
            "render"
        };
        if let Err(e) = world.try_run_schedule(OnExit(old_state.clone())) {
            warn!("({} world) {}", curr_world, e);
        }

        // Update the CurrentState resource before running OnEnter
        let mut current_state_res = world.resource_mut::<CurrentState<T>>();
        current_state_res.val = new_state.clone();

        if let Err(e) = world.try_run_schedule(OnEnter(new_state.clone())) {
            warn!("({} world) {}", curr_world, e);
        }
    }
}
