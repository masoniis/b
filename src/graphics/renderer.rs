use crate::ecs::resources::Camera;
use crate::graphics::shaders::shader_program::ShaderProgram;
use gl::types::{GLfloat, GLsizeiptr, GLuint};
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, WindowSurface};
use std::ffi::c_void;
use std::ptr;

pub struct Renderer {
    vao: GLuint,
    vbo: GLuint,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
}

impl Renderer {
    pub fn new(gl_surface: Surface<WindowSurface>, gl_context: PossiblyCurrentContext) -> Self {
        let mut vao = 0;
        let mut vbo = 0;

        let vertices: [GLfloat; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0, // layout (location = 0)
                3, // size of vertex attribute
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<GLfloat>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Renderer {
            vao,
            vbo,
            gl_surface,
            gl_context,
        }
    }

    pub fn set_frame(&self, shader_program: &ShaderProgram, camera: &Camera) {
        unsafe {
            // Clear previous buffer
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Use the shader program and set camera uniforms
            shader_program.activate(); // Ensure shader is active
            shader_program.set_mat4("modelView", &camera.get_view_matrix());
            shader_program.set_mat4("projection", &camera.get_projection_matrix());

            // Add the triangle
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
