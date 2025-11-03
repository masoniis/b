pub mod core;
pub mod main_camera_centric;
pub mod ui_pass;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        passes::{
            core::execute_render_graph_system, main_camera_centric::PlayerCentricRenderPassPlugin,
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
            .add_plugin(PlayerCentricRenderPassPlugin)
            .add_plugin(UiRenderPassPlugin);

        // INFO: ----------------
        //         Render
        // ----------------------

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(execute_render_graph_system.in_set(RenderSet::Render));
    }
}
