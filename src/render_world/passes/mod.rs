pub mod core;
pub mod ui_pass;
pub mod world;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        passes::{
            core::execute_render_graph_system, ui_pass::UiRenderPassPlugin,
            world::WorldRenderPassesPlugin,
        },
        scheduling::{RenderSchedule, RenderSet},
    },
};
use bevy_ecs::schedule::IntoScheduleConfigs;

/// A plugin that sets up all the necessary resources and render
/// passes used in the rendering pipeline.
pub struct RenderPassManagerPlugin;

impl Plugin for RenderPassManagerPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_plugin(WorldRenderPassesPlugin)
            .add_plugin(UiRenderPassPlugin);

        // INFO: ----------------
        //         render
        // ----------------------

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(execute_render_graph_system.in_set(RenderSet::Render));
    }
}
