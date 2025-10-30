pub mod core;
pub mod opaque_pass;
pub mod transparent_pass;
pub mod ui_pass;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{
    ecs_core::{
        state_machine::{in_state, AppState},
        EcsBuilder, Plugin,
    },
    render_world::{
        passes::{
            core::{
                execute_render_graph_system, setup_view_bind_group_layout_system,
                view::{
                    camera_view_buffer::setup_camera_view_buffer_system,
                    update_camera_view_buffer_system,
                },
            },
            opaque_pass::OpaqueRenderPassPlugin,
            transparent_pass::TransparentRenderPassPlugin,
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
        // renderpass plugins
        builder
            .add_plugin(TransparentRenderPassPlugin)
            .add_plugin(OpaqueRenderPassPlugin)
            .add_plugin(UiRenderPassPlugin);

        // INFO: -----------------
        //         Startup
        // -----------------------

        // these startup schedules are shared by multiple passes

        builder.schedule_entry(RenderSchedule::Startup).add_systems(
            (
                setup_view_bind_group_layout_system,
                setup_camera_view_buffer_system,
            )
                .chain(),
        );

        // INFO: -----------------
        //         Prepare
        // -----------------------

        builder.schedule_entry(RenderSchedule::Main).add_systems(
            update_camera_view_buffer_system
                .run_if(in_state(AppState::Running))
                .in_set(RenderSet::Prepare),
        );

        // INFO: ----------------
        //         Render
        // ----------------------

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(execute_render_graph_system.in_set(RenderSet::Render));
    }
}
