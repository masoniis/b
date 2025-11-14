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
        passes::world::main_passes::{
            bounding_box_pass::{
                extract::WireframeToggleExtractor,
                queue::queue_wireframe_system,
                startup::{setup_bb_pipeline_and_buffers, setup_unit_bounding_box_mesh_system},
            },
            shared_resources,
        },
        RenderSchedule, RenderSet,
    },
    simulation_world::block::TargetedBlock,
};
use bevy_ecs::schedule::IntoScheduleConfigs;

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
                setup_bb_pipeline_and_buffers
                    .after(shared_resources::setup_central_camera_layout_system),
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

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(queue_wireframe_system.in_set(RenderSet::Queue));
    }
}
