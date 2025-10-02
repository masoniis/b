use super::super::State;
use bevy_ecs::prelude::*;

#[derive(Resource, Debug)]
pub struct NextState<T: State> {
    pub value: Option<T>,
}
