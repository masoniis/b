pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        passes::opaque_pass::{
            prepare::{
                MainTextureBindGroup, MeshPipelineLayoutsResource, ModelBindGroup, ViewBindGroup,
            },
            queue::Opaque3dRenderPhase,
        },
        scheduling::{RenderSchedule, RenderSet},
    },
};
use bevy_ecs::prelude::*;

pub struct OpaqueRenderPassPlugin;

impl Plugin for OpaqueRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .init_resource::<MeshPipelineLayoutsResource>()
            .init_resource::<ViewBindGroup>()
            .init_resource::<MainTextureBindGroup>()
            .init_resource::<ModelBindGroup>();

        // INFO: -----------------
        //         Startup
        // -----------------------

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
            // resources
            .init_resource::<Opaque3dRenderPhase>()
            // systems
            .schedule_entry(RenderSchedule::Main)
            .add_systems(queue::queue_mesh_system.in_set(RenderSet::Queue));
    }
}
