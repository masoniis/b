use crate::simulation_world::graphics_old::components::visibility::VisibilityComponent;
use crate::simulation_world::input::{
    types::simulation_action::SimulationAction, ActionStateResource,
};
use crate::simulation_world::screen_text::DiagnosticUiElementMarker;
use bevy_ecs::prelude::*;

pub fn toggle_debug_diagnostics_system(
    action_state: Res<ActionStateResource>,
    mut query: Query<&mut VisibilityComponent, With<DiagnosticUiElementMarker>>,
) {
    if action_state.just_happened(SimulationAction::ToggleDiagnostics) {
        for mut visibility in query.iter_mut() {
            visibility.toggle()
        }
    }
}
