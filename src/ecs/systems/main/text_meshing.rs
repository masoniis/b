use crate::ecs::components::MeshComponent;
use crate::ecs::components::ScreenTextComponent;
use crate::ecs::systems::startup::font_loader::FontAtlas;
use crate::graphics::buffers::Buffer;
use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use glam::Vec2;
use tracing::info;

const FONT_ATLAS_ID: &str = "font_atlas";

/// System to generate/update meshes for text entities
pub fn update_text_mesh_system(
    mut commands: Commands,
    query: Query<(Entity, &ScreenTextComponent)>,
    font_atlas: Res<FontAtlas>,
) {
    // TODO: It is unnecessary to redo this every frame, perhaps use a dirty bit for text meshing?

    for (entity, text) in &query {
        let font = font_atlas.fonts.get("Inter").unwrap();
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            ..LayoutSettings::default()
        });
        layout.append(&[font], &TextStyle::new(&text.text, text.font_size, 0));

        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut index_offset = 0;

        for glyph in layout.glyphs() {
            if let Some((uv_min, uv_max)) = font_atlas
                .glyph_cache
                .get(&(glyph.parent, text.font_size as u32))
            {
                let x = text.position.x + glyph.x;
                let y = text.position.y + glyph.y;
                let w = glyph.width as f32;
                let h = glyph.height as f32;

                // 0 --- 1
                // |  \  |
                // 3 --- 2

                // 0: top-left
                vertices.extend_from_slice(&[x, y, uv_min.x, uv_min.y]);
                // 1: top-right
                vertices.extend_from_slice(&[x + w, y, uv_max.x, uv_min.y]);
                // 2: bottom-right
                vertices.extend_from_slice(&[x + w, y + h, uv_max.x, uv_max.y]);
                // 3: bottom-left
                vertices.extend_from_slice(&[x, y + h, uv_min.x, uv_max.y]);

                indices.extend_from_slice(&[
                    index_offset,
                    index_offset + 2,
                    index_offset + 1,
                    index_offset,
                    index_offset + 3,
                    index_offset + 2,
                ]);
                index_offset += 4;
            }
        }

        if !vertices.is_empty() {
            info!(
                "Updating text mesh for entity {:?}: '{}'",
                entity, text.text
            );
            let buffer = Buffer::new_2d(&vertices, &indices);
            commands.entity(entity).insert(MeshComponent {
                buffer,
                atlas_id: FONT_ATLAS_ID.to_string(),
                uv_min: Vec2::ZERO, // Not used for text meshes
                uv_max: Vec2::ONE,  // Not used for text meshes
            });
        }
    }
}
