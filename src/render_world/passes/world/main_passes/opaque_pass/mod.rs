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
        passes::world::main_passes::{
            opaque_pass::{
                extract::OpaqueRenderModeExtractor,
                queue::Opaque3dRenderPhase,
                startup::{setup_opaque_buffers_and_bind_groups, setup_opaque_pipelines},
            },
            shared_resources::setup_central_camera_layout_system,
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
                setup_opaque_pipelines.after(setup_central_camera_layout_system),
                setup_opaque_buffers_and_bind_groups,
            )
                .chain(),
        );

        // INFO: -----------------
        //         Extract
        // -----------------------

        builder
            .add_plugin(ExtractComponentPlugin::<OpaqueMeshComponent>::default())
            .schedule_entry(RenderSchedule::Extract)
            .add_systems(extract_resource_system::<OpaqueRenderModeExtractor>);

        // INFO: -----------------
        //         Prepare
        // -----------------------

        builder.schedule_entry(RenderSchedule::Main).add_systems(
            prepare::prepare_opaque_meshes_system
                .run_if(in_state(AppState::Running))
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
