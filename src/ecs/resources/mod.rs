pub mod camera;
pub use camera::CameraResource;

pub mod input;
pub use input::InputResource;

pub mod texture_manager;
pub use texture_manager::TextureManagerResource;

pub mod time;
pub use time::TimeResource;

pub mod window;
pub use window::WindowResource;

pub mod shader_manager;
pub use shader_manager::{ShaderManagerResource, ShaderType};
