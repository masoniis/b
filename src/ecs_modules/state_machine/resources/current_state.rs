use super::super::State;
use bevy_ecs::prelude::*;

#[derive(Resource, Debug)]
pub struct CurrentState<T: State> {
    pub value: T,
}
