use crate::game_world::input::{ActionStateResource, GameAction};
use crate::game_world::{graphics::VisibilityComponent, screen_text::DiagnosticUiElementMarker};
use bevy_ecs::prelude::*;

pub fn toggle_debug_diagnostics_system(
    action_state: Res<ActionStateResource>,
    mut query: Query<&mut VisibilityComponent, With<DiagnosticUiElementMarker>>,
) {
    if action_state.just_happened(GameAction::ToggleDiagnostics) {
        for mut visibility in query.iter_mut() {
            visibility.toggle()
        }
    }
}
