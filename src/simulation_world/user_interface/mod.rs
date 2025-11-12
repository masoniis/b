pub mod components;
pub mod layout;
pub mod screens;
pub mod text;

// INFO: ----------------
//         Plugin
// ----------------------

use self::layout::handle_window_resize_system;
use crate::simulation_world::scheduling::StartupSet;
use crate::simulation_world::user_interface::screens::{
    DebugScreenPlugin, GameScreenPlugin, LoadingScreenPlugin,
};
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{SimulationSchedule, SimulationSet},
};
use bevy_ecs::prelude::*;
use {
    layout::{
        compute_and_apply_layout_system, compute_ui_depth_system, handle_hierarchy_changes_system,
        handle_structural_changes_system, update_changed_styles_system, EntityToNodeMap,
        IsLayoutDirty, UiLayoutTree,
    },
    screens::spawn_ui_root_system,
    text::setup_font_system,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -------------------
        //         Resources
        // -------------------------

        builder.world.init_non_send_resource::<UiLayoutTree>();
        builder
            .add_resource(EntityToNodeMap::default())
            .add_resource(IsLayoutDirty::default());

        // INFO: -----------------
        //         Plugins
        // -----------------------

        builder
            .add_plugin(LoadingScreenPlugin)
            .add_plugin(DebugScreenPlugin)
            .add_plugin(GameScreenPlugin);

        // INFO: -----------------
        //         Systems
        // -----------------------

        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(
                (setup_font_system, spawn_ui_root_system)
                    .in_set(StartupSet::ResourceInitialization)
                    .chain(),
            );

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((
                (handle_window_resize_system,).in_set(SimulationSet::Update),
                (
                    handle_structural_changes_system,
                    handle_hierarchy_changes_system,
                    update_changed_styles_system,
                )
                    .chain()
                    .in_set(SimulationSet::PostUpdate),
                (compute_and_apply_layout_system, compute_ui_depth_system)
                    .run_if(resource_equals(IsLayoutDirty(true)))
                    .in_set(SimulationSet::RenderPrep),
            ));
    }
}
