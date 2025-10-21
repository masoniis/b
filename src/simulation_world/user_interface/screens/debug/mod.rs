pub mod debug_screen;
pub mod mesh_counter;
pub mod update_fps_counter;

pub use debug_screen::{diagnostic_ui_is_visible, toggle_debug_diagnostics_system};
pub use mesh_counter::{update_mesh_counter_system, update_mesh_stats_system};
pub use update_fps_counter::update_fps_counter_system;

// INFO: ----------------
//         Plugin
// ----------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        input::{ActionStateResource, SimulationAction},
        user_interface::screens::debug::mesh_counter::MeshCounterResource,
        SimulationSchedule,
    },
};
use bevy_ecs::prelude::*;

pub struct DebugScreenPlugin;

impl Plugin for DebugScreenPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.world.init_resource::<MeshCounterResource>();

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((
                toggle_debug_diagnostics_system.run_if(|action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::ToggleDiagnostics)
                }),
                update_mesh_stats_system,
                (update_fps_counter_system, update_mesh_counter_system)
                    .after(toggle_debug_diagnostics_system)
                    .run_if(diagnostic_ui_is_visible),
            ));
    }
}
