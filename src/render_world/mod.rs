use crate::{
    core::{graphics::context::GraphicsContext, world::CommonEcsInterface},
    prelude::*,
};
use bevy_ecs::schedule::ScheduleLabel;
use extract::ExtractModulePlugin;
use pipeline::{GraphicsContextResource, PipelineModulePlugin};
use queue::plugin::QueueModulePlugin;
use std::ops::{Deref, DerefMut};

pub mod extract;
pub mod pipeline;
pub mod queue;

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

    // TODO: add system sets and stuff to the core schedules

    builder
        .add_plugin(PipelineModulePlugin)
        .add_plugin(QueueModulePlugin)
        .add_plugin(ExtractModulePlugin);

    builder
}

pub fn build_render_world(
    mut builder: EcsBuilder,
    graphics_context: GraphicsContext,
) -> RenderWorldInterface {
    // Add gfx context as a resource
    builder.add_resource(GraphicsContextResource {
        context: graphics_context,
    });

    // Drain all the schedules from the plugins build steps
    for (_, schedule) in builder.schedules.drain_schedules() {
        builder.world.add_schedule(schedule);
    }

    RenderWorldInterface {
        common: CommonEcsInterface {
            world: builder.world,
        },
    }
}
