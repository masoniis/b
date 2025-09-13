use crate::ecs::components::{Mesh, Transform};
use crate::ecs::resources::Camera;
use crate::graphics::renderer::Renderer;
use crate::graphics::shader_program::ShaderProgram;
use bevy_ecs::prelude::{NonSendMut, Query, Res};

pub fn render_system(
    camera: Res<Camera>,
    query: Query<(&Mesh, &Transform)>,
    renderer: NonSendMut<Renderer>, // main-thread only (NonSend)
    shader_program: NonSendMut<ShaderProgram>,
) {
    renderer.clear_frame();

    shader_program.activate();
    shader_program.set_mat4("view", &camera.get_view_matrix());
    shader_program.set_mat4("projection", &camera.get_projection_matrix());

    for (mesh, transform) in &query {
        shader_program.set_mat4("model", &transform.to_matrix());

        if let Some(ref texture) = mesh.texture {
            texture.bind(0); // Bind to texture unit 0
            shader_program.set_int("u_texture", 0);
        }

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

    renderer.swap_buffers();
}
