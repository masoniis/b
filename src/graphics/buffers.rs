use gl::types::{GLfloat, GLsizeiptr, GLuint};
use std::ffi::c_void;
use std::ptr;

pub struct Buffer {
    pub vao: GLuint,
    pub vbo: GLuint,
}

impl Buffer {
    pub fn new(vertices: &[GLfloat]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;

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

        Buffer { vao, vbo }
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
