pub mod main_passes;
pub mod shadow_pass;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::passes::world::{
        main_passes::PlayerCentricRenderPassPlugin, shadow_pass::ShadowRenderPassPlugin,
    },
};

/// A plugin that sets up all the necessary resources and render
/// passes used in the rendering pipeline.
pub struct WorldRenderPassesPlugin;

impl Plugin for WorldRenderPassesPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // renderpass plugins
        builder
            .add_plugin(PlayerCentricRenderPassPlugin)
            .add_plugin(ShadowRenderPassPlugin);
    }
}
