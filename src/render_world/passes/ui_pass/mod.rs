pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

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
        RenderSchedule,
    },
};
use bevy_ecs::prelude::*;
use prepare::{PreparedUiBatches, UiRenderBatch};

pub struct RenderUiPlugin;

impl Plugin for RenderUiPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.init_resource::<ExtractedBy<UiPanelExtractor>>();
        builder.init_resource::<ExtractedBy<UiTextExtractor>>();
        builder.init_resource::<PreparedUiBatches>();
        builder
            .world
            .insert_resource(RenderPhase::<UiRenderBatch>::default());

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

        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                extract::extract_component_system::<UiPanelExtractor>,
                extract::extract_component_system::<UiTextExtractor>,
            ));

        builder.schedule_entry(RenderSchedule::Prepare).add_systems(
            (
                prepare::prepare_ui_batches_system.run_if(
                    resource_changed::<ExtractedBy<UiPanelExtractor>>
                        .or(resource_changed::<ExtractedBy<UiTextExtractor>>),
                ),
                prepare::prepare_ui_view_system
                    .run_if(resource_changed::<RenderWindowSizeResource>),
            )
                .chain(),
        );

        builder
            .schedule_entry(RenderSchedule::Queue)
            .add_systems(queue::queue_ui_system);
    }
}
