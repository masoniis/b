use crate::prelude::*;
use bevy_ecs::prelude::*;
use glyphon::{fontdb::Source, FontSystem};
use std::sync::Arc;

#[derive(Resource)]
pub struct FontSystemResource {
    pub font_system: FontSystem,
}

const FONT_PATH: &str = "assets/fonts/Miracode.ttf";

/// A startup system to load and insert font state
pub fn setup_font_system(mut commands: Commands) {
    let font_bytes = std::fs::read(FONT_PATH).expect("Failed to load font");
    let source = Source::Binary(Arc::new(font_bytes));
    let font_system = FontSystem::new_with_fonts(vec![source]);

    info!("Font system initialized with font at {}", FONT_PATH);

    commands.insert_resource(FontSystemResource { font_system });
}
