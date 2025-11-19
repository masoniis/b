pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::TransparentPassRenderNode;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::EcsBuilder, Plugin,
    render_world::{
        global_extract::ExtractComponentPlugin,
        passes::world::main_passes::transparent_pass::{
            prepare::prepare_transparent_meshes_system,
            queue::{queue_and_prepare_transparent_system, Transparent3dRenderPhase},
            startup::setup_transparent_pass_system,
        },
        RenderSchedule, RenderSet,
        passes::world::main_passes::opaque_pass::prepare::delete_gpu_buffers_system,
    },
    simulation_world::chunk::mesh::TransparentMeshComponent,
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct TransparentRenderPassPlugin;

impl Plugin for TransparentRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         Startup
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Startup)
            .add_systems(setup_transparent_pass_system);

        // INFO: -----------------
        //         Extract
        // -----------------------

        builder.add_plugin(ExtractComponentPlugin::<TransparentMeshComponent>::default());

        // INFO: -----------------
        //         Prepare
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(
                (
                    delete_gpu_buffers_system.before(prepare_transparent_meshes_system),
                    prepare_transparent_meshes_system,
                )
                    .in_set(RenderSet::Prepare),
            );

        // INFO: ---------------
        //         Queue
        // ---------------------

        builder
            // resources
            .init_resource::<Transparent3dRenderPhase>()
            // systems
            .schedule_entry(RenderSchedule::Main)
            .add_systems(queue_and_prepare_transparent_system.in_set(RenderSet::Queue));
    }
}
