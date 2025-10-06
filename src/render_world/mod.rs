pub mod context;
pub mod extract;
pub mod passes;
pub mod plugin;
pub mod resources;
pub mod textures;
pub mod types;
pub mod uniforms;

use crate::{prelude::*, render_world::textures::load_texture_array};
use bevy_ecs::schedule::ScheduleLabel;
use context::GraphicsContext;
use plugin::RenderPlugin;
use resources::{GraphicsContextResource, TextureArrayResource};
use std::ops::{Deref, DerefMut};

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
    let (texture_array, _texture_registry) =
        load_texture_array(&graphics_context.device, &graphics_context.queue).unwrap();

    builder.add_resource(TextureArrayResource {
        array: texture_array,
    });

    builder.add_resource(GraphicsContextResource {
        context: graphics_context,
    });

    builder.add_plugin(RenderPlugin);

    builder
}

/// Builds the final state and returns the final render world interface.
pub fn build_render_world(mut builder: EcsBuilder) -> RenderWorldInterface {
    for (_, schedule) in builder.schedules.drain_schedules() {
        builder.world.add_schedule(schedule);
    }

    RenderWorldInterface {
        common: CommonEcsInterface {
            world: builder.world,
        },
    }
}
