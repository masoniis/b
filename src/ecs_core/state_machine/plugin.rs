use super::{
    apply_state_transition_system, {CurrentState, NextState, PrevState},
};
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{SimulationSchedule, SimulationSet},
};
use bevy_ecs::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

// Trait to bundle all the necessary derives needed for state attributes
pub trait State: Send + Sync + 'static + Copy + Clone + Eq + Hash + Debug + Default {}

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
    fn build(&self, builder: &mut EcsBuilder) {
        builder.world.init_resource::<CurrentState<T>>();
        builder.world.init_resource::<NextState<T>>();
        builder.world.init_resource::<PrevState<T>>();

        // Add the transition system for this specific state type
        builder
            .schedule_entry(SimulationSchedule::Loading)
            .add_systems(apply_state_transition_system::<T>.in_set(SimulationSet::PostUpdate));

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(apply_state_transition_system::<T>.in_set(SimulationSet::PostUpdate));
    }
}
