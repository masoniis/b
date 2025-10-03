use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, ScheduleLabel},
    world::World,
};

use super::state_machine::State;
use crate::game_world::{system_sets::StartupSet, CoreSet};
use std::collections::HashMap;

/// Schedule that runs once in the entering state.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnEnter<T: State>(pub T);

/// Schedule that runs once in the exiting state as we transition to a new state.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnExit<T: State>(pub T);

/// Core pre-defined schedule labels
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleLables {
    Startup,
    Loading,
    Main,
}

/// A container for all the core schedules of the app.
pub struct Schedules {
    /// Runs a single time at app start.
    pub startup: Schedule,
    /// Runs every frame after startup, waiting for loading tasks to complete.
    /// Transitions to `main` schedule when loading is complete.
    pub loading: Schedule,
    /// Runs every frame during normal gameplay.
    pub main: Schedule,

    labeled: HashMap<Box<dyn ScheduleLabel>, Schedule>,
}

impl Schedules {
    pub fn new() -> Self {
        let mut startup_schedule = Schedule::new(ScheduleLables::Startup);
        startup_schedule.configure_sets((StartupSet::InitialSetup, StartupSet::Finalize).chain());

        let mut main_schedule = Schedule::new(ScheduleLables::Main);

        main_schedule.configure_sets(
            (
                CoreSet::Input,
                CoreSet::PreUpdate,
                CoreSet::Update,
                CoreSet::Physics,
                CoreSet::PostUpdate,
                CoreSet::RenderPrep,
                CoreSet::Render,
            )
                .chain(),
        );

        Self {
            startup: startup_schedule,
            loading: Schedule::new(ScheduleLables::Loading),
            main: main_schedule,
            labeled: HashMap::new(),
        }
    }

    pub fn drain_dynamic_schedules(&mut self) -> HashMap<Box<dyn ScheduleLabel>, Schedule> {
        return self.labeled.drain().collect();
    }

    /// Get a labeled schedule and add it if it doesn't exist
    ///
    /// This ensures that any plugin that uses a labeled schedule like
    /// OnEnter will also guarantee the schedule exists in the world.
    pub fn get_labeled_mut(&mut self, label: impl ScheduleLabel + Clone) -> &mut Schedule {
        self.labeled
            .entry(Box::new(label.clone()))
            .or_insert_with(|| Schedule::new(label))
    }
}

/// A trait for that enables a module to plug into the ECS context and inject resources and schedules.
pub trait Plugin {
    /// Builds and adds all systems for this group into the provided schedules.
    fn build(&self, schedules: &mut Schedules, world: &mut World);
}
