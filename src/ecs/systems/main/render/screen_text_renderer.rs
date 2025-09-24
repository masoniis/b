use crate::ecs::components::ScreenTextComponent;
use crate::graphics::GlyphonRenderer;
use bevy_ecs::prelude::{Query, ResMut};

pub fn screen_text_render_system(
    mut _renderer: ResMut<GlyphonRenderer>,
    _query: Query<&ScreenTextComponent>,
) {
    return;
}
