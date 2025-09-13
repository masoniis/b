use crate::ecs::components::Mesh;
use crate::ecs::resources::shader_manager::{ShaderManager, ShaderType};
use crate::ecs::resources::texture_manager::TextureManager;
use crate::ecs::resources::window::WindowResource;
use crate::ecs::systems::startup::font_loader_system::FontAtlas;
use crate::graphics::buffers::Buffer;
use bevy_ecs::prelude::{Commands, Component, Entity, NonSend, Query, Res};
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use glam::{Mat4, Vec2, Vec4};
use tracing::info;

const FONT_ATLAS_ID: &str = "font_atlas";

#[derive(Component)]
pub struct Text {
    pub text: String,
    pub position: Vec2,
    pub font_size: f32,
}

// System to generate/update meshes for text entities
pub fn update_text_mesh_system(
    mut commands: Commands,
    query: Query<(Entity, &Text)>,
    font_atlas: Res<FontAtlas>,
) {
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
            commands.entity(entity).insert(Mesh {
                buffer,
                atlas_id: FONT_ATLAS_ID.to_string(),
                uv_min: Vec2::ZERO, // Not used for text meshes
                uv_max: Vec2::ONE,  // Not used for text meshes
            });
        }
    }
}

pub fn render_text_system(
    query: Query<(&Mesh, &Text)>,
    shader_manager: NonSend<ShaderManager>,
    texture_manager: NonSend<TextureManager>,
    window_size: Res<WindowResource>,
) {
    let shader = shader_manager.get(ShaderType::Text).unwrap();
    shader.activate();

    let projection = Mat4::orthographic_rh_gl(
        0.0,
        window_size.width as f32,
        window_size.height as f32,
        0.0,
        -1.0,
        1.0,
    );
    shader.set_mat4("projection", &projection);

    // Bind font atlas texture
    let font_atlas_texture = texture_manager.get_texture(FONT_ATLAS_ID).unwrap();
    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, font_atlas_texture.id());
    }
    shader.set_int("u_texture", 0);
    shader.set_vec4("u_textColor", &Vec4::new(1.0, 1.0, 1.0, 1.0)); // Set default text color to white

    for (mesh, _text) in query.iter() {
        unsafe {
            gl::BindVertexArray(mesh.buffer.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                mesh.buffer.index_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
}
