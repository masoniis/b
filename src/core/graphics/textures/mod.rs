mod array;
mod names;
mod registry;

pub use array::{load_texture_array, TextureArray};
pub use names::*; // Use a glob import for constants, which is a common pattern.
pub use registry::TextureRegistry;
