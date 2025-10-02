use crate::ecs_modules::state_machine::{resources::CurrentState, State};
use bevy_ecs::prelude::*;

pub fn in_state<T: State>(check_state: T) -> impl Fn(Res<CurrentState<T>>) -> bool {
    move |current_state: Res<CurrentState<T>>| current_state.val == check_state
}
