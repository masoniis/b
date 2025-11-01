pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::OpaquePassRenderNode;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{
        state_machine::{in_state, AppState},
        EcsBuilder, Plugin,
    },
    render_world::{
        global_extract::{extract_resource_system, ExtractComponentPlugin},
        graphics_context::resources::RenderSurfaceConfig,
        passes::{
            core::{self},
            opaque_pass::{
                extract::OpaqueRenderModeExtractor,
                queue::Opaque3dRenderPhase,
                startup::{
                    setup_opaque_buffers_and_bind_groups, setup_opaque_depth_texture_system,
                    setup_opaque_pipelines,
                },
            },
        },
        scheduling::{RenderSchedule, RenderSet},
    },
    simulation_world::chunk::OpaqueMeshComponent,
};
use bevy_ecs::prelude::*;

pub struct OpaqueRenderPassPlugin;

impl Plugin for OpaqueRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         Startup
        // -----------------------
        builder.schedule_entry(RenderSchedule::Startup).add_systems(
            (
                setup_opaque_pipelines.after(core::setup_view_bind_group_layout_system),
                setup_opaque_buffers_and_bind_groups,
                setup_opaque_depth_texture_system,
            )
                .chain(),
        );

        // INFO: -----------------
        //         Extract
        // -----------------------

        builder.add_plugin(ExtractComponentPlugin::<OpaqueMeshComponent>::default());
        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems(extract_resource_system::<OpaqueRenderModeExtractor>);

        // INFO: -----------------
        //         Prepare
        // -----------------------
        builder.schedule_entry(RenderSchedule::Main).add_systems(
            (
                (startup::setup_opaque_depth_texture_system)
                    .run_if(resource_changed_or_removed::<RenderSurfaceConfig>),
                prepare::prepare_opaque_meshes_system.run_if(in_state(AppState::Running)),
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
            .add_systems(queue::queue_and_prepare_opaque_system.in_set(RenderSet::Queue));
    }
}
