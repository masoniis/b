use crate::core::state_machine::State;
use bevy_ecs::schedule::ScheduleLabel;

/// Schedule that runs once in the entering state.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnEnter<T: State>(pub T);

/// Schedule that runs once in the exiting state as we transition to a new state.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnExit<T: State>(pub T);

/// Core pre-defined schedule labels
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSchedule {
    Startup,
    Loading,
    Main,
}
