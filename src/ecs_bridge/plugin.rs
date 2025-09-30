use bevy_ecs::{
    schedule::{Schedule, ScheduleLabel},
    world::World,
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleLables {
    Startup,
    Input, // should run just before main
    Main,
}

/// A container for all the core schedules of the app.
pub struct Schedules {
    pub startup: Schedule,
    pub input: Schedule,
    pub main: Schedule,
}

impl Schedules {
    /// Creates a new instance with empty schedules.
    pub fn new() -> Self {
        Self {
            startup: Schedule::new(ScheduleLables::Startup),
            input: Schedule::new(ScheduleLables::Input),
            main: Schedule::new(ScheduleLables::Main),
        }
    }
}

/// A trait for that enables a module to plug into the ECS context and inject resources and schedules.
pub trait Plugin {
    /// Builds and adds all systems for this group into the provided schedules.
    fn build(&self, schedules: &mut Schedules, world: &mut World);
}
