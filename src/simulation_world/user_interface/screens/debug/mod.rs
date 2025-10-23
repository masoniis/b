pub mod debug_screen;
pub mod elements;

pub use debug_screen::{diagnostic_ui_is_visible, toggle_debug_diagnostics_system};
pub use elements::fps_counter::update_fps_counter_screen_text_system;
pub use elements::mesh_counter::{
    update_mesh_counter_screen_text_system, update_mesh_stats_system, MeshCounterResource,
};

// INFO: ----------------
//         Plugin
// ----------------------

use crate::ecs_core::state_machine::{in_state, AppState};
use crate::simulation_world::user_interface::screens::elements::update_camera_chunk_chord_screen_text;
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
                    (
                        update_camera_chunk_chord_screen_text,
                        toggle_debug_diagnostics_system,
                    )
                        .chain()
                        .run_if(
                            (|action_state: Res<ActionStateResource>| {
                                action_state.just_happened(SimulationAction::ToggleDiagnostics)
                            })
                            .and(in_state(AppState::Running)),
                        ),
                    update_mesh_stats_system,
                    // currently running this every frame so we don't miss the toggle frame
                    // rather than tick-based, and unlike the other systems, it doesn't really
                    // change every frame so we don't need to run it on the tick system anyway
                    update_mesh_counter_screen_text_system
                        .after(update_mesh_stats_system)
                        .run_if(resource_changed::<MeshCounterResource>.or(
                            |action_state: Res<ActionStateResource>| {
                                action_state.just_happened(SimulationAction::ToggleDiagnostics)
                            },
                        )),
                )
                    .in_set(SimulationSet::PostUpdate),
            );

        builder
            .schedule_entry(SimulationSchedule::FixedUpdate)
            .add_systems(
                (
                    update_camera_chunk_chord_screen_text,
                    update_fps_counter_screen_text_system,
                )
                    .run_if(diagnostic_ui_is_visible)
                    .in_set(SimulationSet::PostUpdate),
            );
    }
}
