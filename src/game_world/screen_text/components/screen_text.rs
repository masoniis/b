use bevy_ecs::prelude::Component;
use glam::Vec2;
use glyphon::cosmic_text::Color;

#[derive(Component)]
pub struct ScreenTextComponent {
    pub text: String,
    pub position: Vec2,
    pub color: Color,
    pub font_size: f32,
}

// INFO: --------------------------
//         Component markers
// --------------------------------

#[derive(Component)]
pub struct FpsCounterScreenTextMarker;

/// A component marker for any UI in the
/// diagnostic screen (eg, the fps counter)
#[derive(Component)]
pub struct DiagnosticUiElementMarker;
