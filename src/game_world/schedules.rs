use crate::ecs_core::state_machine::State;
use bevy_ecs::schedule::ScheduleLabel;

/// Schedule that runs once in the entering state.
///
/// It will run before any other system in that state.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnEnter<T: State>(pub T);

/// Schedule that runs once in the exiting state as we transition to a new state.
///
/// This means it will run before any system in the new state we are entering.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnExit<T: State>(pub T);

/// Core pre-defined schedule labels
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSchedule {
    Startup,
    Loading,
    Main,
}
