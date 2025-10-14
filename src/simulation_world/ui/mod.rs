pub mod components;
pub mod creation;
pub mod layout;
pub mod text;

// INFO: ----------------
//         Plugin
// ----------------------

use crate::ecs_core::{EcsBuilder, Plugin};
use crate::simulation_world::ui::layout::handle_window_resize_system;
use crate::simulation_world::{SimulationSchedule, SimulationSet};
use bevy_ecs::prelude::*;
use {
    creation::create_test_ui_system,
    layout::{
        compute_and_apply_layout_system, compute_ui_depth_system, handle_hierarchy_changes_system,
        handle_structural_changes_system, update_changed_styles_system, EntityToNodeMap,
        IsLayoutDirty, UiLayoutTree,
    },
    text::setup_font_system,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.world.init_non_send_resource::<UiLayoutTree>();

        builder
            .add_resource(EntityToNodeMap::default())
            .add_resource(IsLayoutDirty::default());

        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems((create_test_ui_system, setup_font_system));

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((
                (
                    handle_window_resize_system,
                    (
                        handle_structural_changes_system,
                        handle_hierarchy_changes_system,
                        update_changed_styles_system,
                        handle_window_resize_system,
                    )
                        .chain(),
                )
                    .in_set(SimulationSet::Update),
                (compute_and_apply_layout_system, compute_ui_depth_system)
                    .run_if(resource_equals(IsLayoutDirty(true)))
                    .in_set(SimulationSet::RenderPrep),
            ));
    }
}
