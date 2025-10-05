use crate::core::state_machine::State;
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
}
impl State for GameState {}
