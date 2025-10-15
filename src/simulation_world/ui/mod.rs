pub mod components;
pub mod layout;
pub mod screens;
pub mod text;

// INFO: ----------------
//         Plugin
// ----------------------

use crate::ecs_core::{EcsBuilder, Plugin};
use crate::simulation_world::app_lifecycle::AppState;
use crate::simulation_world::input::{ActionStateResource, SimulationAction};
use crate::simulation_world::ui::layout::handle_window_resize_system;
use crate::simulation_world::ui::screens::debug::toggle_debug_diagnostics_system;
use crate::simulation_world::ui::screens::loading_screen::despawn_loading_ui_system;
use crate::simulation_world::{OnExit, SimulationSchedule, SimulationSet};
use bevy_ecs::prelude::*;
use {
    layout::{
        compute_and_apply_layout_system, compute_ui_depth_system, handle_hierarchy_changes_system,
        handle_structural_changes_system, update_changed_styles_system, EntityToNodeMap,
        IsLayoutDirty, UiLayoutTree,
    },
    screens::{spawn_loading_ui_system, spawn_ui_root_system},
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
            .add_systems(
                (
                    (setup_font_system, spawn_ui_root_system),
                    spawn_loading_ui_system,
                )
                    .chain(),
            );

        builder
            .schedule_entry(OnExit(AppState::Loading))
            .add_systems(despawn_loading_ui_system);

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((
                (
                    handle_window_resize_system,
                    toggle_debug_diagnostics_system.run_if(
                        |action_state: Res<ActionStateResource>| {
                            action_state.just_happened(SimulationAction::ToggleDiagnostics)
                        },
                    ),
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
