pub mod environment_uniform;
pub mod main_depth_texture;
pub mod tex_array_uniforms;
pub mod view_uniforms;

pub use environment_uniform::{
    update_environment_uniform_buffer_system, EnvironmentBindGroupLayout, EnvironmentUniforms,
};
pub use main_depth_texture::{
    resize_main_depth_texture_system, MainDepthTextureResource, MAIN_DEPTH_FORMAT,
};
pub use tex_array_uniforms::{TextureArrayBindGroupLayout, TextureArrayUniforms};
pub use view_uniforms::{
    update_camera_view_buffer_system, CentralCameraViewBindGroupLayout, CentralCameraViewUniform,
};
