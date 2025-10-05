use crate::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use context::GraphicsContext;
use extract::ExtractModulePlugin;
use prepare::plugin::PrepareModulePlugin;
use queue::plugin::QueueModulePlugin;
use render::{GraphicsContextResource, PipelineModulePlugin};
use std::ops::{Deref, DerefMut};

pub mod context;
pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderSchedule {
    Extract,
    Prepare,
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

pub fn configure_render_world(graphics_context: GraphicsContext) -> EcsBuilder {
    let mut builder = EcsBuilder::new();

    // As a root resource that requites input from app, graphics context must be
    // inserted before we do any other system building.
    //
    // This is because systems can't create it themselves like most other resources.
    builder.add_resource(GraphicsContextResource {
        context: graphics_context,
    });

    builder
        .add_plugin(PipelineModulePlugin)
        .add_plugin(QueueModulePlugin)
        .add_plugin(ExtractModulePlugin)
        .add_plugin(PrepareModulePlugin);

    builder
}

pub fn build_render_world(mut builder: EcsBuilder) -> RenderWorldInterface {
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
