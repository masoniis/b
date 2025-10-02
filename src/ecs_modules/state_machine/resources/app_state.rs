use crate::ecs_modules::state_machine::State;
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Running,
    Closing,
}
impl State for AppState {}
