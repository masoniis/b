pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::UiRenderPassNode;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        global_extract::resources::RenderWindowSizeResource,
        passes::core::{self},
        passes::ui_pass::{
            extract::ExtractedUiEvents,
            prepare::UiChanges,
            queue::{
                IsGlyphonDirty, PreparedUiBatches, UiElementCache, UiElementSortBufferResource,
            },
        },
        scheduling::{RenderSchedule, RenderSet},
    },
};
use bevy_ecs::prelude::*;

pub struct UiRenderPassPlugin;

impl Plugin for UiRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         Startup
        // -----------------------

        builder.schedule_entry(RenderSchedule::Startup).add_systems(
            (
                startup::setup_ui_pipeline.after(core::setup_view_bind_group_layout_system),
                startup::setup_ui_unit_quad_system,
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
            .add_resource(ExtractedUiEvents::default())
            // systems
            .schedule_entry(RenderSchedule::Extract)
            .add_systems(extract::extract_ui_events_system);

        // INFO: -----------------
        //         Prepare
        // -----------------------

        builder
            // resources
            .init_resource::<PreparedUiBatches>()
            .init_resource::<UiElementSortBufferResource>()
            .init_resource::<IsGlyphonDirty>()
            .init_resource::<UiChanges>()
            .init_resource::<UiElementCache>()
            // schedule
            .schedule_entry(RenderSchedule::Main)
            .add_systems(
                (
                    (
                        prepare::update_ui_view_data_system,
                        prepare::prepare_glyphon_view_system,
                    )
                        .run_if(resource_changed::<RenderWindowSizeResource>),
                    (prepare::process_ui_events_system,).chain(),
                )
                    .in_set(RenderSet::Prepare),
            );

        // INFO: ---------------
        //         Queue
        // ---------------------

        builder.schedule_entry(RenderSchedule::Main).add_systems(
            (
                // make decisions based on the UiChanges determined above
                (
                    queue::mark_glyphon_dirty_system,
                    queue::rebuild_ui_batches_system,
                ),
                // makes changes based on the buffers from the systems just before it
                queue::preprocess_glyphon_text_system.run_if(resource_equals(IsGlyphonDirty(true))),
            )
                .in_set(RenderSet::Queue)
                .chain(),
        );
    }
}
