use gl::types::{GLchar, GLenum, GLint, GLuint};
use tracing::{error, info};

use std::{ffi::CString, fs::File, io::Read, ptr, str};

/// Represents a compiled and linked shader program.
pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    /// Creates a new shader program from vertex and fragment shader source files.
    ///
    /// # Arguments
    ///
    /// * `vertex_shader_path` - The file path to the vertex shader source code.
    /// * `fragment_shader_path` - The file path to the fragment shader source code.
    ///
    /// # Errors
    ///
    /// Returns an `Err` with a descriptive message if the shader files cannot be
    /// opened or read, or if shader compilation or linking fails.
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Result<Self, String> {
        let mut vertex_shader_file = File::open(vertex_shader_path)
            .map_err(|e| format!("Failed to open vertex shader: {}", e))?;
        let mut fragment_shader_file = File::open(fragment_shader_path)
            .map_err(|e| format!("Failed to open fragment shader: {}", e))?;

        let mut vertex_shader_source = String::new();
        let mut fragment_shader_source = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_source)
            .map_err(|e| format!("Failed to read vertex shader: {}", e))?;
        fragment_shader_file
            .read_to_string(&mut fragment_shader_source)
            .map_err(|e| format!("Failed to read fragment shader: {}", e))?;

        let vertex_shader = Self::compile_shader(&vertex_shader_source, gl::VERTEX_SHADER)?;
        let fragment_shader = Self::compile_shader(&fragment_shader_source, gl::FRAGMENT_SHADER)?;

        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);
        }

        let mut success = gl::FALSE as GLint;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == gl::TRUE as GLint {
            unsafe {
                gl::DeleteShader(vertex_shader);
                gl::DeleteShader(fragment_shader);
            }
            info!("Successfully created shader program");
            Ok(ShaderProgram { id })
        } else {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buffer = Vec::with_capacity(len as usize);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error_message = unsafe {
                gl::GetProgramInfoLog(id, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
                CString::from_vec_unchecked(buffer)
            };
            error!(
                "Failed to link shader program: {}",
                error_message.to_string_lossy()
            );
            Err("Failed to link shader program".to_string())
        }
    }

    /// Compiles the shader and returns the shader ID or an error message
    fn compile_shader(source: &str, shader_type: GLenum) -> Result<GLuint, String> {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);
        }

        let mut success = gl::FALSE as GLint;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        }

        if success == gl::TRUE as GLint {
            Ok(shader)
        } else {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buffer = Vec::with_capacity(len as usize);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error_message = unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                CString::from_vec_unchecked(buffer)
            };
            error!(
                "Failed to compile shader: {}",
                error_message.to_string_lossy()
            );
            Err("Failed to compile shader".to_string())
        }
    }

    /// Activates the shader program for use in the current rendering context.
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// Deletes the shader program, freeing its resources.
    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

