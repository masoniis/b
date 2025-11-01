pub mod queue;
pub mod render;
pub mod startup;

pub use render::WireframeRenderNode;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        passes::wireframe_pass::{
            queue::queue_wireframe_system,
            startup::{setup_wireframe_mesh_system, setup_wireframe_pipeline_and_buffers},
        },
        RenderSchedule, RenderSet,
    },
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct WireframeRenderPassPlugin;

impl Plugin for WireframeRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         Startup
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Startup)
            .add_systems((
                setup_wireframe_mesh_system,
                setup_wireframe_pipeline_and_buffers,
            ));

        // INFO: ---------------
        //         Queue
        // ---------------------

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(queue_wireframe_system.in_set(RenderSet::Queue));
    }
}
