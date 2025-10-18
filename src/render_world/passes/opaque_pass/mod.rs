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
        passes::opaque_pass::queue::Opaque3dRenderPhase,
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
