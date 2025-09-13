use gl::types::*;
use image::{self, GenericImageView};
use std::ffi::c_void;
use tracing::info;

pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(path: &str) -> Result<Self, String> {
        info!("Loading texture from {}", path);
        let img = image::open(path)
            .map_err(|e| format!("Failed to load texture {}: {}", path, e))?
            .flipv(); // OpenGL expects textures to be upside down

        let (width, height) = img.dimensions();
        let data = img.to_rgba8();
        if data.is_empty() {
            return Err(format!("Texture data is empty for {}", path));
        }

        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Self { id, width, height })
    }

    pub fn bind(&self, unit: GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
