pub mod setup_buffers;
pub mod setup_depth_texture;
pub mod setup_pipeline;

pub use setup_buffers::*;
pub use setup_depth_texture::*;
pub use setup_pipeline::*;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
