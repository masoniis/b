pub mod extract;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::BoundingBoxNode;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        global_extract::{clone_resource_system, extract_resource_system},
        passes::main_camera_centric::{
            bounding_box_pass::{
                extract::{WireframeToggleExtractor, WireframeToggleState},
                queue::{clear_wireframe_buffer_system, queue_wireframe_system},
                startup::{setup_bb_pipeline_and_buffers, setup_unit_bounding_box_mesh_system},
            },
            shared,
        },
        RenderSchedule, RenderSet,
    },
    simulation_world::block::TargetedBlock,
};
use bevy_ecs::schedule::{common_conditions::resource_equals, IntoScheduleConfigs};

pub struct WireframeRenderPassPlugin;

impl Plugin for WireframeRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         Startup
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Startup)
            .add_systems((
                setup_unit_bounding_box_mesh_system,
                setup_bb_pipeline_and_buffers.after(shared::setup_central_camera_layout_system),
            ));

        // INFO: -----------------
        //         Extract
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                extract_resource_system::<WireframeToggleExtractor>,
                clone_resource_system::<TargetedBlock>,
            ));

        // INFO: ---------------
        //         Queue
        // ---------------------

        builder.schedule_entry(RenderSchedule::Main).add_systems((
            queue_wireframe_system
                .in_set(RenderSet::Queue)
                .run_if(resource_equals(WireframeToggleState { enabled: true })),
            clear_wireframe_buffer_system
                .run_if(resource_equals(WireframeToggleState { enabled: false })),
        ));
    }
}
