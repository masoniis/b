use gl::types::{GLfloat, GLsizeiptr, GLuint};
use std::ffi::c_void;
use std::ptr;

pub struct Renderer {
    vao: GLuint,
    vbo: GLuint,
}

impl Renderer {
    pub fn new() -> Self {
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

        Renderer { vao, vbo }
    }

    pub fn draw_triangle(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
