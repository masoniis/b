mod array;
pub mod error;
pub mod registry;
pub mod resource;

pub use array::{load_texture_array, GpuTextureArray};
pub use error::TextureLoadError;
pub use registry::TextureRegistry;
pub use resource::TextureArrayResource;
