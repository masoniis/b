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
    pub fn from_bytes(bytes: &[u8], width: u32, height: u32) -> Result<Self, String> {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                width as i32,
                height as i32,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                bytes.as_ptr() as *const c_void,
            );

            // Reset pixel store alignment to default
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 4);
        }

        Ok(Self { id, width, height })
    }

    pub fn dump_to_file(&self, path: &str) -> Result<(), String> {
        let mut pixels = vec![0u8; (self.width * self.height) as usize];
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                pixels.as_mut_ptr() as *mut c_void,
            );
            gl::PixelStorei(gl::PACK_ALIGNMENT, 4); // Reset to default
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        image::save_buffer(path, &pixels, self.width, self.height, image::ColorType::L8)
            .map_err(|e| format!("Failed to save texture to {}: {}", path, e))?;

        info!("Texture dumped to {}", path);
        Ok(())
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
