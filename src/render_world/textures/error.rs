use std::fmt;

#[derive(Debug)]
pub enum TextureLoadError {
    DirectoryRead(String, std::io::Error),
    ImageError(String, image::ImageError),
    NoTexturesFound,
    DimensionMismatch(String, u32, u32, u32, u32),
    MissingTextureNotInManifest,
}

impl fmt::Display for TextureLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextureLoadError::DirectoryRead(path, err) => {
                write!(f, "Failed to read texture directory at '{}': {}", path, err)
            }
            TextureLoadError::ImageError(path, err) => {
                write!(f, "Failed to open or decode image at '{}': {}", path, err)
            }
            TextureLoadError::NoTexturesFound => {
                write!(f, "No valid textures found to determine dimensions.")
            }
            TextureLoadError::DimensionMismatch(name, w, h, exp_w, exp_h) => {
                write!(
                    f,
                    "Texture '{}' has dimensions {}x{}, but expected {}x{}.",
                    name, w, h, exp_w, exp_h
                )
            }
            TextureLoadError::MissingTextureNotInManifest => {
                write!(
                    f,
                    "The required TextureId::Missing was not found in the manifest."
                )
            }
        }
    }
}

impl std::error::Error for TextureLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TextureLoadError::DirectoryRead(_, err) => Some(err),
            TextureLoadError::ImageError(_, err) => Some(err),
            _ => None,
        }
    }
}
