pub use crate::ecs_modules::{
    state_machine::resources::{CurrentState, NextState},
    state_machine::types::State,
};
use bevy_ecs::prelude::*;

pub fn apply_state_transition_system<T: State>(
    mut current_state: ResMut<CurrentState<T>>,
    mut next_state: ResMut<NextState<T>>,
) {
    if let Some(new_state) = next_state.value.take() {
        if new_state != current_state.value {
            current_state.value = new_state;
        }
    }
}
