pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::OpaquePassRenderNode;
use startup::OpaquePipelines;

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
        passes::world::main_passes::opaque_pass::{
            extract::OpaqueRenderModeExtractor, queue::Opaque3dRenderPhase,
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

        builder.init_resource::<OpaquePipelines>();

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
            (
                prepare::delete_gpu_buffers_system.before(prepare::prepare_opaque_meshes_system),
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
            .add_systems(queue::queue_opaque_system.in_set(RenderSet::Queue));
    }
}
