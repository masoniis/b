use crate::{
    ecs_modules::{rendering::VisibilityComponent, screen_text::DiagnosticUiElementMarker},
    ecs_resources::InputResource,
    prelude::*,
};
use bevy_ecs::prelude::*;

pub fn toggle_debug_diagnostics_system(
    input: Res<InputResource>,
    mut query: Query<&mut VisibilityComponent, With<DiagnosticUiElementMarker>>,
) {
    if input.was_key_pressed(KeyCode::KeyR) {
        for mut visibility in query.iter_mut() {
            visibility.toggle()
        }
    }
}
