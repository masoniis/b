use crate::ecs_modules::input::InputResource;
use crate::ecs_modules::{rendering::VisibilityComponent, screen_text::DiagnosticUiElementMarker};
use bevy_ecs::prelude::*;
use winit::keyboard::{KeyCode, PhysicalKey};

pub fn toggle_debug_diagnostics_system(
    input: Res<InputResource>,
    mut query: Query<&mut VisibilityComponent, With<DiagnosticUiElementMarker>>,
) {
    if input.was_key_pressed(PhysicalKey::Code(KeyCode::KeyR)) {
        for mut visibility in query.iter_mut() {
            visibility.toggle()
        }
    }
}
