use super::super::State;
use bevy_ecs::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct NextState<T: State> {
    pub val: Option<T>, // value conflicts with the Res namespace from bevy and the LSP doesn't like it so using val
}
