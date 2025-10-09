pub mod context;
pub mod extract;
pub mod material;
pub mod passes;
pub mod plugin;
pub mod resources;
pub mod textures;
pub mod types;
pub mod uniforms;

use crate::{
    ecs_core::worlds::RenderWorldMarker, prelude::*, render_world::textures::load_texture_array,
};
use bevy_ecs::schedule::ScheduleLabel;
use context::GraphicsContext;
use passes::setup_render_graph;
use plugin::RenderPlugin;
use resources::{GraphicsContextResource, TextureArrayResource};
use std::ops::{Deref, DerefMut};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderSchedule {
    Startup,
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

impl RenderWorldInterface {
    /// Creates a new render world with a sane default configuration
    pub fn new(graphics_context: GraphicsContext) -> Self {
        let mut builder = EcsBuilder::new();

        // As a root resource that requites input from app, graphics context must be
        // inserted before we do any other system building.
        //
        // This is because systems can't create it themselves like most other resources.
        let (texture_array, _texture_registry) =
            load_texture_array(&graphics_context.device, &graphics_context.queue).unwrap();

        // Setup render graph runs as an early system since it needs mutable world access
        setup_render_graph(&mut builder.world);

        // Add any resources that require specific app input
        builder
            .add_resource(TextureArrayResource {
                array: texture_array,
            })
            .add_resource(GraphicsContextResource {
                context: graphics_context,
            })
            .add_resource(RenderWorldMarker);

        // And finally add the plugins
        builder.add_plugin(RenderPlugin);

        return Self::build_render_world(builder);
    }

    /// Builds the final state and returns the final render world interface.
    fn build_render_world(mut builder: EcsBuilder) -> RenderWorldInterface {
        for (_, schedule) in builder.schedules.drain_schedules() {
            builder.world.add_schedule(schedule);
        }

        RenderWorldInterface {
            common: CommonEcsInterface {
                world: builder.world,
            },
        }
    }
}
