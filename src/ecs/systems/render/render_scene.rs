use crate::ecs::components::{MeshComponent, TransformComponent};
use crate::ecs::resources::{Camera, ShaderManager, ShaderType, TextureManager};
use bevy_ecs::prelude::{NonSend, NonSendMut, Query, Res};

/// System responsible for facilitating the rendering of the 3D scene
pub fn render_scene_system(
    camera: Res<Camera>,
    query: Query<(&MeshComponent, &TransformComponent)>,
    mut shader_manager: NonSendMut<ShaderManager>,
    texture_manager: NonSend<TextureManager>,
) {
    if let Some(shader) = shader_manager.get_mut(ShaderType::Scene) {
        shader.activate();
        shader.set_mat4("view", &camera.get_view_matrix());
        shader.set_mat4("projection", &camera.get_projection_matrix());

        // Bind the main atlas once
        if let Some(main_atlas) = texture_manager.get_texture("main_atlas") {
            main_atlas.bind(0); // Bind to texture unit 0
            shader.set_int("u_texture", 0);
        } else {
            // Handle error: main_atlas not found
            eprintln!("Error: 'main_atlas' not found in TextureManager!");
            return;
        }

        for (mesh, transform) in &query {
            shader.set_mat4("model", &transform.to_matrix());

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
    } else {
        panic!("Scene shader not found in ShaderManager but we are trying to render the scene!");
    }
}
