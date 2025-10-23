pub mod debug_screen;
pub mod mesh_counter;
pub mod update_fps_counter;

pub use debug_screen::{diagnostic_ui_is_visible, toggle_debug_diagnostics_system};
pub use mesh_counter::{
    update_mesh_counter_screen_text_system, update_mesh_stats_system, MeshCounterResource,
};
pub use update_fps_counter::update_fps_counter_screen_text_system;

// INFO: ----------------
//         Plugin
// ----------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        input::{ActionStateResource, SimulationAction},
        SimulationSchedule,
    },
    SimulationSet,
};
use bevy_ecs::prelude::*;

pub struct DebugScreenPlugin;

impl Plugin for DebugScreenPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.world.init_resource::<MeshCounterResource>();

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (
                    toggle_debug_diagnostics_system.run_if(
                        |action_state: Res<ActionStateResource>| {
                            action_state.just_happened(SimulationAction::ToggleDiagnostics)
                        },
                    ),
                    update_mesh_stats_system,
                    (
                        update_fps_counter_screen_text_system,
                        update_mesh_counter_screen_text_system
                            .after(update_mesh_stats_system)
                            .run_if(resource_changed::<MeshCounterResource>.or(
                                |action_state: Res<ActionStateResource>| {
                                    action_state.just_happened(SimulationAction::ToggleDiagnostics)
                                },
                            )),
                    )
                        .after(toggle_debug_diagnostics_system)
                        .run_if(diagnostic_ui_is_visible),
                )
                    .in_set(SimulationSet::PostUpdate),
            );
    }
}
