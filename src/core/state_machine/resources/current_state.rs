use super::super::State;
use bevy_ecs::prelude::*;

#[derive(Resource, Debug, Default, PartialEq, Eq)]
pub struct CurrentState<T: State> {
    pub val: T, // value conflicts with the Res namespace from bevy and the LSP doesn't like it so using val
}
