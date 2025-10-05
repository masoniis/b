use super::{
    apply_state_transition_system, State, {CurrentState, NextState, PrevState},
};
use crate::{
    game_world::{schedules::GameSchedule, Plugin, ScheduleBuilder},
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;
use std::marker::PhantomData;

/// A generic plugin for any type T that implements the State trait
pub struct StatePlugin<T: State>(PhantomData<T>);

impl<T: State> Default for StatePlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Generic implementation just adds state transition systems for the
/// state type to the the the main schedules that want/need them
impl<T: State> Plugin for StatePlugin<T> {
    fn build(&self, schedules: &mut ScheduleBuilder, world: &mut World) {
        world.init_resource::<CurrentState<T>>();
        world.init_resource::<NextState<T>>();
        world.init_resource::<PrevState<T>>();

        // Add the transition system for this specific state type
        schedules
            .entry(GameSchedule::Loading)
            .add_systems(apply_state_transition_system::<T>.in_set(CoreSet::PostUpdate));

        schedules
            .entry(GameSchedule::Main)
            .add_systems(apply_state_transition_system::<T>.in_set(CoreSet::PostUpdate));
    }
}
