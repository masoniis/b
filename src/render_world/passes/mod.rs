pub mod core;
pub mod opaque_pass;
pub mod ui_pass;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        passes::{
            core::render_graph_system, opaque_pass::OpaqueRenderPassPlugin,
            ui_pass::UiRenderPassPlugin,
        },
        scheduling::{RenderSchedule, RenderSet},
    },
};

/// A plugin that sets up all the necessary resources and render
/// passes used in the rendering pipeline.
pub struct RenderPassManagerPlugin;

impl Plugin for RenderPassManagerPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_plugin(OpaqueRenderPassPlugin)
            .add_plugin(UiRenderPassPlugin);

        // INFO: -----------------
        //         Startup
        // -----------------------
        // these startup schedules are shared
        // by all the render passes

        builder
            .schedule_entry(RenderSchedule::Startup)
            .add_systems(core::setup_view_bind_group_layout_system);

        // INFO: -----------------
        //         Extract
        // -----------------------

        // INFO: -----------------
        //         Prepare
        // -----------------------

        // INFO: ---------------
        //         Queue
        // ---------------------
        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(render_graph_system.in_set(RenderSet::Render));
    }
}
