pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;
pub mod utils;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use self::queue::RenderPhase;
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        extract::{
            self,
            extract_component::ExtractedBy,
            ui::{UiPanelExtractor, UiTextExtractor},
            RenderWindowSizeResource,
        },
        passes::ui_pass::{prepare::UiElementSortBufferResource, utils::ui_was_extracted},
        RenderSchedule,
    },
};
use bevy_ecs::prelude::*;
use prepare::{PreparedUiBatches, UiRenderBatch};

pub struct RenderUiPlugin;

impl Plugin for RenderUiPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         Startup
        // -----------------------

        builder.schedule_entry(RenderSchedule::Startup).add_systems(
            (
                startup::setup_view_bind_group_layout,
                startup::setup_ui_pipeline,
                startup::setup_ui_screen_quad_system,
                startup::setup_ui_buffers,
                startup::setup_glyphon_resources,
            )
                .chain(),
        );

        // INFO: -----------------
        //         Extract
        // -----------------------

        builder
            // resources
            .init_resource::<ExtractedBy<UiPanelExtractor>>()
            .init_resource::<ExtractedBy<UiTextExtractor>>()
            // systems
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                extract::extract_component_system::<UiPanelExtractor>,
                extract::extract_component_system::<UiTextExtractor>,
            ));

        // INFO: -----------------
        //         Prepare
        // -----------------------

        builder
            // resources
            .init_resource::<PreparedUiBatches>()
            .init_resource::<UiElementSortBufferResource>();
        builder
            .world
            .insert_resource(RenderPhase::<UiRenderBatch>::default());

        builder.schedule_entry(RenderSchedule::Prepare).add_systems(
            (
                prepare::prepare_ui_batches_system.run_if(ui_was_extracted),
                (
                    prepare::prepare_ui_view_system,
                    prepare::prepare_glyphon_view_system,
                )
                    .run_if(resource_changed::<RenderWindowSizeResource>),
            )
                .chain(),
        );

        builder
            .schedule_entry(RenderSchedule::Queue)
            .add_systems(queue::queue_ui_system);
    }
}
