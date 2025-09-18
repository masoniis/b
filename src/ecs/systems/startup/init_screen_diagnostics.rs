use crate::ecs::components::{
    DiagnosticUiElementMarker, FpsCounterScreenTextMarker, ScreenTextComponent, VisibilityComponent,
};
use bevy_ecs::prelude::Commands;
use glam::vec2;

pub fn init_screen_diagnostics_system(mut commands: Commands) {
    const FONT_SIZE: f32 = 48.0;

    commands.spawn((
        ScreenTextComponent {
            text: "FPS: 69.0".to_string(),
            position: vec2(24.0, 96.0),
            font_size: FONT_SIZE,
        },
        VisibilityComponent::Visible,
        FpsCounterScreenTextMarker,
        DiagnosticUiElementMarker,
    ));

    commands.spawn((
        ScreenTextComponent {
            text: "Debug Stats".to_string(),
            position: vec2(24.0, 24.0),
            font_size: FONT_SIZE,
        },
        VisibilityComponent::Visible,
        DiagnosticUiElementMarker,
    ));
}
