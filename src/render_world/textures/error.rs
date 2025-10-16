use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureLoadError {
    #[error("Failed to read texture directory at '{0}': {1}")]
    DirectoryRead(String, std::io::Error),
    #[error("Failed to open or decode image at '{0}': {1}")]
    ImageError(String, image::ImageError),
    #[error("No valid textures found to determine dimensions.")]
    NoTexturesFound,
    #[error("Texture '{0}' has dimensions {1}x{2}, but expected {3}x{4}.")]
    DimensionMismatch(String, u32, u32, u32, u32),
    #[error("The required TextureId::Missing was not found in the manifest.")]
    MissingTextureNotInManifest,
}
