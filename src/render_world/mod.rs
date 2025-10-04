use crate::{
    core::{graphics::context::GraphicsContext, world::CommonEcsInterface},
    prelude::*,
};
use bevy_ecs::schedule::{Schedule, ScheduleLabel};
use std::ops::{Deref, DerefMut};

pub mod extract;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderSchedule {
    Extract,
    Queue,
    Render,
}

// INFO: --------------------------------
//         Render world interface
// --------------------------------------

pub struct RenderWorldInterface {
    pub common: CommonEcsInterface,
    pub graphics_context: GraphicsContext,
}

impl Deref for RenderWorldInterface {
    type Target = CommonEcsInterface;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for RenderWorldInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

// INFO: ---------------------------
//         Render World Builder
// ---------------------------------

pub fn configure_render_world() -> EcsBuilder {
    let mut builder = EcsBuilder::new();

    // Ensure render schedules exist before plugins are added.
    let mut extract_schedule = Schedule::new(RenderSchedule::Extract);
    // extract_schedule.add_systems(extract_meshes_systes);

    let mut queue_schedule = Schedule::new(RenderSchedule::Queue);
    // queue_schedule.add_systems(queue_meshes_system);

    builder.world.add_schedule(extract_schedule);
    builder.world.add_schedule(queue_schedule);
    builder
        .world
        .add_schedule(Schedule::new(RenderSchedule::Render));

    // builder.add_plugins(RenderPlugins); // Example: Add render-specific plugins here
    builder
}

pub fn build_render_world(
    mut builder: EcsBuilder,
    graphics_context: GraphicsContext,
) -> RenderWorldInterface {
    for (_, schedule) in builder.schedules.drain_schedules() {
        builder.world.add_schedule(schedule);
    }

    RenderWorldInterface {
        graphics_context,
        common: CommonEcsInterface {
            world: builder.world,
        },
    }
}
