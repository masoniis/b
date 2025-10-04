use crate::{
    core::{graphics::context::GraphicsContext, world::CommonEcsInterface},
    prelude::*,
};
use std::ops::{Deref, DerefMut};

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
    let mut _builder = EcsBuilder::new();

    // Ensure render schedules exist before plugins are added.

    // builder.schedules.get_mut(RenderSchedule::Extract);
    // builder.schedules.get_mut(RenderSchedule::Prepare);
    // builder.schedules.get_mut(RenderSchedule::Queue);
    // builder.schedules.get_mut(RenderSchedule::Render);
    // builder.schedules.get_mut(RenderSchedule::Cleanup);

    // builder.add_plugins(RenderPlugins); // Example: Add render-specific plugins here
    _builder
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
