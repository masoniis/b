use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, ScheduleLabel},
};
use std::collections::HashMap;

use tracing::warn;

// INFO: --------------------------------
//         Generic ECS Primitives
// --------------------------------------

/// A generic container to collect schedules.
///
/// When a bunch of schedules have been
/// collected, they can be drained by the
/// builder to be injected into an ecs world.
#[derive(Default)]
pub struct ScheduleBuilder {
    labeled: HashMap<Box<dyn ScheduleLabel>, Schedule>,
}

impl ScheduleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Drain all the schedules that have been added to the builder.
    pub fn drain_schedules(&mut self) -> HashMap<Box<dyn ScheduleLabel>, Schedule> {
        self.labeled.drain().collect()
    }

    /// Gets the current builder entry for a schedule or creates it if it doesn't exist
    pub fn entry(&mut self, label: impl ScheduleLabel + Clone) -> &mut Schedule {
        self.labeled
            .entry(Box::new(label.clone()))
            .or_insert_with(|| Schedule::new(label))
    }
}

/// A trait that enables a module to plug into the ECS context.
pub trait Plugin {
    fn build(&self, schedules: &mut ScheduleBuilder, world: &mut World);
}

/// A trait for composing groups of plugins.
pub trait PluginGroup {
    fn build(self, builder: &mut EcsBuilder);
}

/// Generic ECS interface builder
pub struct EcsBuilder {
    pub world: World,
    pub schedules: ScheduleBuilder,
}

impl EcsBuilder {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            schedules: ScheduleBuilder::new(),
        }
    }

    pub fn add_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(&mut self.schedules, &mut self.world);
        self
    }

    pub fn add_plugins<G: PluginGroup>(&mut self, group: G) -> &mut Self {
        group.build(self);
        self
    }
}

// INFO: --------------------------
//         Shared interface
// --------------------------------

/// An interface for the app to safely interact with any ECS world
pub struct CommonEcsInterface {
    pub world: World,
}

impl CommonEcsInterface {
    pub fn run_schedule(&mut self, label: impl ScheduleLabel + Clone) {
        match self.world.try_run_schedule(label.clone()) {
            Ok(_) => {}
            Err(error) => {
                warn!(
                    "Schedule with label {:?} not found or failed to run: {}",
                    label.dyn_clone(),
                    error
                );
            }
        }
    }
}
