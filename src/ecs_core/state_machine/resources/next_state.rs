use super::super::State;
use bevy_ecs::prelude::*;

#[derive(Resource, Debug, Default, PartialEq, Eq)]
pub struct NextState<T: State> {
    pub val: Option<T>, // 'value' as a name conflicts with the Res namespace from bevy and the LSP doesn't like it so using 'val'
}
