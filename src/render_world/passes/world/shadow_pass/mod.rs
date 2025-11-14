pub mod extract;
pub mod prepare;
pub mod render;
pub mod startup;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        global_extract::extract_resource_system,
        passes::world::shadow_pass::{
            extract::SunExtractor,
            prepare::update_shadow_view_buffer_system,
            startup::{
                setup_shadow_depth_texture_system, setup_shadow_pass_pipeline,
                setup_shadow_view_buffer_system,
            },
        },
        RenderSchedule,
    },
    RenderSet,
};

pub struct ShadowRenderPassPlugin;

impl Plugin for ShadowRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         startup
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Startup)
            .add_systems((
                // depth texture
                setup_shadow_depth_texture_system,
                // view buffer
                (setup_shadow_pass_pipeline, setup_shadow_view_buffer_system).chain(),
            ));

        // INFO: -----------------
        //         extract
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems(extract_resource_system::<SunExtractor>);

        // INFO: -----------------
        //         prepare
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(update_shadow_view_buffer_system.in_set(RenderSet::Prepare));
    }
}
