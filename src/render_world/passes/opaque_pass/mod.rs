pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{
        state_machine::{in_state, AppState},
        EcsBuilder, Plugin,
    },
    render_world::{
        graphics_context::resources::RenderSurfaceConfig,
        passes::{
            core::{self},
            opaque_pass::queue::Opaque3dRenderPhase,
        },
        scheduling::{RenderSchedule, RenderSet},
    },
};
use bevy_ecs::prelude::*;

pub struct OpaqueRenderPassPlugin;

impl Plugin for OpaqueRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         Startup
        // -----------------------
        builder
            .schedule_entry(RenderSchedule::Startup)
            .add_systems(((
                startup::setup_opaque_pipeline.after(core::setup_view_bind_group_layout_system),
                startup::setup_opaque_buffers_and_bind_groups,
                startup::setup_depth_texture_system,
            )
                .chain(),));

        // INFO: -----------------
        //         Extract
        // -----------------------

        // INFO: -----------------
        //         Prepare
        // -----------------------
        builder.schedule_entry(RenderSchedule::Main).add_systems(
            (
                (startup::setup_depth_texture_system)
                    .run_if(resource_changed_or_removed::<RenderSurfaceConfig>),
                (
                    prepare::update_opaque_view_data_system,
                    prepare::prepare_meshes_system,
                )
                    .run_if(in_state(AppState::Running)),
            )
                .in_set(RenderSet::Prepare),
        );

        // INFO: ---------------
        //         Queue
        // ---------------------
        builder
            // resources
            .init_resource::<Opaque3dRenderPhase>()
            // systems
            .schedule_entry(RenderSchedule::Main)
            .add_systems(queue::queue_mesh_system.in_set(RenderSet::Queue));
    }
}
