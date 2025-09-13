use crate::ecs::components::{Mesh, Transform};
use crate::ecs::resources::Camera;
use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use bevy_ecs::prelude::{NonSendMut, Query, Res};

pub fn render_system(
    camera: Res<Camera>, // Asks for read-only access to the Camera resource
    query: Query<(&Mesh, &Transform)>, // Asks for a query over Mesh and Transform components
    renderer: NonSendMut<Renderer>, // Asks for mutable, main-thread-only access to Renderer
    mut shader_program: NonSendMut<ShaderProgram>, // Same for ShaderProgram
) {
    renderer.set_frame(&mut shader_program, &camera);
    shader_program.set_mat4("modelView", &camera.get_view_matrix());
    shader_program.set_mat4("projection", &camera.get_projection_matrix());

    // for (mesh, transform) in &query {
    //     // println!("Rendering mesh at position: {:?}", transform.position);
    // }
}
