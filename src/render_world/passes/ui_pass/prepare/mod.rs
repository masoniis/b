pub mod prepare_screen_quad;
pub mod prepare_ui_view;
pub mod systems;

pub use prepare_screen_quad::{prepare_screen_quad_system, ScreenQuadResource};
pub use prepare_ui_view::prepare_ui_view_system;
pub use systems::*;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        extract::{self, extract_component::ExtractedItems, ui::UiNodeExtractor},
        passes::ui_pass::{prepare, queue},
        RenderSchedule,
    },
};
use bevy_ecs::prelude::*;

pub struct RenderUiPlugin;

impl Plugin for RenderUiPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.init_resource::<ExtractedItems<UiNodeExtractor>>();

        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems(extract::extract_component_system::<UiNodeExtractor>);

        builder
            .schedule_entry(RenderSchedule::Prepare)
            .add_systems((
                // (
                //     prepare::prepare_render_buffers_system,
                //     prepare::prepare_pipelines_system,
                // ),
                (
                    prepare::setup_ui_pipeline,
                    prepare::prepare_screen_quad_system,
                    prepare::prepare_ui_nodes_system,
                    prepare::prepare_ui_view_system,
                )
                    .chain(),
            ));

        builder
            .schedule_entry(RenderSchedule::Queue)
            .add_systems(queue::queue_ui_system);
    }
}
