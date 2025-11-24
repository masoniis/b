pub mod bounding_box_pass;
pub mod opaque_pass;
pub mod shared_resources;
pub mod transparent_pass;

use shared_resources::{
    CentralCameraViewBindGroupLayout, EnvironmentBindGroupLayout, TextureArrayBindGroupLayout,
    TextureArrayUniforms,
};
pub use shared_resources::{
    CentralCameraViewUniform, EnvironmentUniforms, MainDepthTextureResource, MAIN_DEPTH_FORMAT,
};

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{
        state_machine::{in_state, AppState},
        EcsBuilder, Plugin,
    },
    render_world::{
        global_extract::RenderWindowSizeResource,
        graphics_context::reconfigure_wgpu_surface_system,
        passes::world::main_passes::{
            bounding_box_pass::WireframeRenderPassPlugin,
            opaque_pass::OpaqueRenderPassPlugin,
            shared_resources::{
                resize_main_depth_texture_system, update_camera_view_buffer_system,
                update_environment_uniform_buffer_system,
            },
            transparent_pass::TransparentRenderPassPlugin,
        },
        scheduling::{RenderSchedule, RenderSet},
    },
};
use bevy_ecs::schedule::{common_conditions::resource_changed_or_removed, IntoScheduleConfigs};

/// A plugin that sets up all the necessary resources and render
/// passes used in the rendering pipeline.
pub struct PlayerCentricRenderPassPlugin;

impl Plugin for PlayerCentricRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: ----------------------------------------------------
        //         startup (shared resources for main passes)
        // ----------------------------------------------------------

        builder
            // camera view uniform resources
            .init_resource::<CentralCameraViewBindGroupLayout>()
            .init_resource::<CentralCameraViewUniform>()
            // environment uniform resources
            .init_resource::<EnvironmentBindGroupLayout>()
            .init_resource::<EnvironmentUniforms>()
            // texture uniform resources
            .init_resource::<TextureArrayBindGroupLayout>()
            .init_resource::<TextureArrayUniforms>()
            // main depth texture
            .init_resource::<MainDepthTextureResource>();

        // INFO: -----------------------------------------
        //         prepare (also shared resources)
        // -----------------------------------------------

        builder.schedule_entry(RenderSchedule::Main).add_systems(
            (
                resize_main_depth_texture_system
                    .run_if(resource_changed_or_removed::<RenderWindowSizeResource>)
                    .after(reconfigure_wgpu_surface_system),
                (
                    update_camera_view_buffer_system,
                    update_environment_uniform_buffer_system,
                )
                    .run_if(in_state(AppState::Running)),
            )
                .in_set(RenderSet::Prepare),
        );

        // INFO: --------------------------------------
        //         subplugins for render passes
        // --------------------------------------------

        builder
            .add_plugin(TransparentRenderPassPlugin)
            .add_plugin(OpaqueRenderPassPlugin)
            .add_plugin(WireframeRenderPassPlugin);
    }
}
