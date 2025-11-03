pub mod opaque_pass;
pub mod shared;
pub mod transparent_pass;
pub mod wireframe_pass;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{
        state_machine::{in_state, AppState},
        EcsBuilder, Plugin,
    },
    render_world::{
        passes::main_camera_centric::shared::{
            setup_central_camera_buffer_system, setup_central_camera_layout_system,
            setup_environment_buffer_system, setup_environment_layout_system,
            shared_environment_buffer::prepare_environment_buffer_system,
            update_camera_view_buffer_system,
        },
        scheduling::{RenderSchedule, RenderSet},
    },
};
use bevy_ecs::schedule::IntoScheduleConfigs;
use opaque_pass::OpaqueRenderPassPlugin;
use transparent_pass::TransparentRenderPassPlugin;
use wireframe_pass::WireframeRenderPassPlugin;

/// A plugin that sets up all the necessary resources and render
/// passes used in the rendering pipeline.
pub struct PlayerCentricRenderPassPlugin;

impl Plugin for PlayerCentricRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // renderpass plugins
        builder
            .add_plugin(TransparentRenderPassPlugin)
            .add_plugin(OpaqueRenderPassPlugin)
            .add_plugin(WireframeRenderPassPlugin);

        // INFO: -----------------
        //         Startup
        // -----------------------

        // these startup schedules are shared by multiple passes

        builder
            .schedule_entry(RenderSchedule::Startup)
            .add_systems((
                (
                    // camera uniform
                    setup_central_camera_layout_system,
                    setup_central_camera_buffer_system,
                )
                    .chain(),
                (
                    // environment uniform
                    setup_environment_layout_system,
                    setup_environment_buffer_system,
                    prepare_environment_buffer_system, // TODO: remove as this should be dynamic
                )
                    .chain(),
            ));

        // INFO: -----------------
        //         Prepare
        // -----------------------

        builder.schedule_entry(RenderSchedule::Main).add_systems(
            update_camera_view_buffer_system
                .run_if(in_state(AppState::Running))
                .in_set(RenderSet::Prepare),
        );
    }
}
