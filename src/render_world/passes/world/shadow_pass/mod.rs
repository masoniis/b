pub mod gpu_resources;
pub mod prepare;
pub mod render;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use bevy_ecs::schedule::IntoScheduleConfigs;
use gpu_resources::{
    shadow_view_uniform::ShadowViewBindGroupLayout, ShadowDepthTextureResource, ShadowPassPipeline,
    ShadowViewBuffer,
};

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        passes::world::shadow_pass::prepare::update_shadow_view_buffer_system, RenderSchedule,
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
            .init_resource::<ShadowViewBindGroupLayout>()
            .init_resource::<ShadowPassPipeline>()
            .init_resource::<ShadowViewBuffer>()
            .init_resource::<ShadowDepthTextureResource>();

        // INFO: -----------------
        //         prepare
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(update_shadow_view_buffer_system.in_set(RenderSet::Prepare));
    }
}
