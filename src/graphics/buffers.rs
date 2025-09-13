use gl::types::{GLfloat, GLsizeiptr, GLuint};
use std::ffi::c_void;
use std::ptr;

pub struct Buffer {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
    pub index_count: GLuint,
}

impl Buffer {
    pub fn new(vertices: &[GLfloat], indices: &[GLuint]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = 5 * std::mem::size_of::<GLfloat>() as i32;

            // Position attribute
            gl::VertexAttribPointer(
                0, // layout (location = 0)
                3, // size of vertex attribute
                gl::FLOAT,
                gl::FALSE,
                stride,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Texture coordinate attribute
            gl::VertexAttribPointer(
                1, // layout (location = 1)
                2, // size of vertex attribute
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * std::mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Buffer {
            vao,
            vbo,
            ebo,
            index_count: indices.len() as GLuint,
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
