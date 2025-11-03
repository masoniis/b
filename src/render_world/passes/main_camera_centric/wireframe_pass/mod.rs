pub mod extract;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::WireframeRenderNode;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        global_extract::extract_resource_system,
        passes::main_camera_centric::{
            shared::setup_central_camera_layout_system,
            wireframe_pass::{
                extract::{WireframeToggleExtractor, WireframeToggleState},
                queue::{clear_wireframe_buffer_system, queue_wireframe_system},
                startup::{setup_wireframe_mesh_system, setup_wireframe_pipeline_and_buffers},
            },
        },
        RenderSchedule, RenderSet,
    },
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
                setup_wireframe_mesh_system,
                setup_wireframe_pipeline_and_buffers.after(setup_central_camera_layout_system),
            ));

        // INFO: -----------------
        //         Extract
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems(extract_resource_system::<WireframeToggleExtractor>);

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
