use super::super::{DiagnosticUiElementMarker, FpsCounterScreenTextMarker, ScreenTextComponent};
use crate::ecs::modules::rendering::VisibilityComponent;
use bevy_ecs::prelude::Commands;
use glam::vec2;
use glyphon::cosmic_text::Color;

pub fn init_screen_diagnostics_system(mut commands: Commands) {
    const FONT_SIZE: f32 = 48.0;

    commands.spawn((
        ScreenTextComponent {
            text: "FPS: 69.0".to_string(),
            position: vec2(24.0, 96.0),
            font_size: FONT_SIZE,
            color: Color::rgb(0xFF, 0xAF, 0xFF),
        },
        VisibilityComponent::Visible,
        FpsCounterScreenTextMarker,
        DiagnosticUiElementMarker,
    ));

    commands.spawn((
        ScreenTextComponent {
            text: "Debug stats".to_string(),
            position: vec2(24.0, 24.0),
            font_size: FONT_SIZE,
            color: Color::rgb(0xAF, 0xFF, 0xAF),
        },
        VisibilityComponent::Visible,
        DiagnosticUiElementMarker,
    ));
}
