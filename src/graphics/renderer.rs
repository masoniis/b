use crate::ecs::resources::Camera;
use crate::graphics::buffers::Buffer;
use crate::graphics::shaders::shader_program::ShaderProgram;
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, WindowSurface};

pub struct Renderer {
    buffer: Buffer,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
}

impl Renderer {
    pub fn new(gl_surface: Surface<WindowSurface>, gl_context: PossiblyCurrentContext) -> Self {
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];

        let buffer = Buffer::new(&vertices);

        Renderer {
            buffer,
            gl_surface,
            gl_context,
        }
    }

    pub fn set_frame(&self, shader_program: &ShaderProgram, camera: &Camera) {
        unsafe {
            // Clear previous buffer
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Use the shader program and set camera uniforms
            shader_program.activate(); // Ensure shader is active
            shader_program.set_mat4("modelView", &camera.get_view_matrix());
            shader_program.set_mat4("projection", &camera.get_projection_matrix());

            // Add the triangle
            gl::BindVertexArray(self.buffer.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }

    pub fn cleanup(&self) {
        self.buffer.cleanup();
    }
}
