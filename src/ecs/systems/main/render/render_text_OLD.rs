use crate::ecs::components::{MeshComponent, ScreenTextComponent, VisibilityComponent};
use crate::ecs::resources::shader_manager::{ShaderManagerResource, ShaderType};
use crate::ecs::resources::texture_manager::TextureManagerResource;
use crate::ecs::resources::window::WindowResource;
use bevy_ecs::prelude::{NonSend, Query, Res};
use glam::{Mat4, Vec4};

const FONT_ATLAS_ID: &str = "font_atlas";

/// System responsible for facilitating the rendering of the 2D text entities
pub fn render_text_system(
    query: Query<(&MeshComponent, &ScreenTextComponent, &VisibilityComponent)>,
    shader_manager: NonSend<ShaderManagerResource>,
    texture_manager: NonSend<TextureManagerResource>,
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
    shader.set_vec4("u_textColor", &Vec4::new(1.0, 1.0, 1.0, 1.0)); // default white text

    for (mesh, _text, visibility) in query.iter() {
        if *visibility == VisibilityComponent::Visible {
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
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
}
