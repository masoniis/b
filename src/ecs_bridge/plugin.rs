use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, ScheduleLabel},
    world::World,
};

use crate::ecs_modules::CoreSet;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleLables {
    Startup,
    Main,
}

/// A container for all the core schedules of the app.
pub struct Schedules {
    pub startup: Schedule,
    pub main: Schedule,
}

impl Schedules {
    /// Creates a new instance with empty schedules.
    pub fn new() -> Self {
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
            startup: Schedule::new(ScheduleLables::Startup),
            main: main_schedule,
        }
    }
}

/// A trait for that enables a module to plug into the ECS context and inject resources and schedules.
pub trait Plugin {
    /// Builds and adds all systems for this group into the provided schedules.
    fn build(&self, schedules: &mut Schedules, world: &mut World);
}
