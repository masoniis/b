use crate::ecs::resources::TextureManagerResource;
use crate::graphics::textures::Texture;
use bevy_ecs::prelude::{Commands, NonSendMut, Resource};
use fontdue::Font;
use glam::{vec2, Vec2};
use std::collections::HashMap;

const FONT_ATLAS_ID: &str = "font_atlas";

#[derive(Resource)]
pub struct FontAtlas {
    pub fonts: HashMap<String, Font>,
    pub glyph_cache: HashMap<(char, u32), (Vec2, Vec2)>, // (char, size) -> (uv_min, uv_max)
    pub texture_id: String,
}

pub fn font_loader_system(
    mut commands: Commands,
    mut texture_manager: NonSendMut<TextureManagerResource>,
) {
    const FONT_SIZE: f32 = 48.0;
    const CHARACTERS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

    let font_bytes = include_bytes!("../../../assets/fonts/Inter_variable.ttf");
    let font = Font::from_bytes(font_bytes as &[u8], Default::default()).unwrap();

    let mut fonts = HashMap::new();
    fonts.insert("Inter".to_string(), font.clone());

    // Rasterize characters and create the atlas
    let mut rasterized_chars = HashMap::new();
    let mut atlas_width = 0;
    let mut atlas_height = 0;

    for char in CHARACTERS.chars() {
        let (metrics, bitmap) = font.rasterize(char, FONT_SIZE);
        if metrics.width > 0 && metrics.height > 0 {
            atlas_width += metrics.width;
            atlas_height = atlas_height.max(metrics.height);
            rasterized_chars.insert(char, (metrics, bitmap));
        }
    }

    let mut atlas_data = vec![0; atlas_width * atlas_height];
    let mut glyph_cache = HashMap::new();
    let mut current_x = 0;

    for char in CHARACTERS.chars() {
        if let Some((metrics, bitmap)) = rasterized_chars.get(&char) {
            if metrics.width > 0 && metrics.height > 0 {
                for y in 0..metrics.height {
                    for x in 0..metrics.width {
                        let atlas_index = (y + 0) * atlas_width + (current_x + x);
                        let bitmap_index = y * metrics.width + x;
                        atlas_data[atlas_index] = bitmap[bitmap_index];
                    }
                }

                let uv_min = vec2(current_x as f32 / atlas_width as f32, 0.0);
                let uv_max = vec2(
                    (current_x + metrics.width) as f32 / atlas_width as f32,
                    metrics.height as f32 / atlas_height as f32,
                );

                glyph_cache.insert((char, FONT_SIZE as u32), (uv_min, uv_max));
                current_x += metrics.width;
            }
        }
    }

    let texture =
        Texture::from_bytes(&atlas_data, atlas_width as u32, atlas_height as u32).unwrap();

    // Dump the atlas to a file for debugging
    if let Err(e) = texture.dump_to_file("./src/assets/fonts/font_atlas.png") {
        tracing::error!("Failed to dump font atlas: {}", e);
    }

    texture_manager.add_atlas(FONT_ATLAS_ID.to_string(), texture);

    let font_atlas = FontAtlas {
        fonts,
        glyph_cache,
        texture_id: FONT_ATLAS_ID.to_string(),
    };

    commands.insert_resource(font_atlas);
}
