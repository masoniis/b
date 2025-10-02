use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, ScheduleLabel},
    world::World,
};

use crate::ecs_modules::{system_sets::StartupSet, CoreSet};

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
            )
                .chain(),
        );

        Self {
            startup: startup_schedule,
            loading: Schedule::new(ScheduleLables::Loading),
            main: main_schedule,
        }
    }
}

/// A trait for that enables a module to plug into the ECS context and inject resources and schedules.
pub trait Plugin {
    /// Builds and adds all systems for this group into the provided schedules.
    fn build(&self, schedules: &mut Schedules, world: &mut World);
}
