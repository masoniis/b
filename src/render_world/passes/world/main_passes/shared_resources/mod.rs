pub mod main_depth_texture;
pub mod shared_environment_buffer;
pub mod shared_view_buffer;

pub use main_depth_texture::{
    resize_main_depth_texture_system, setup_main_depth_texture_system, MainDepthTextureResource,
    MAIN_DEPTH_FORMAT,
};
pub use shared_environment_buffer::{
    setup_environment_buffer_system, setup_environment_layout_system,
    update_environment_buffer_system, EnvironmentBindGroupLayout, EnvironmentBuffer,
};
pub use shared_view_buffer::{
    setup_central_camera_buffer_system, setup_central_camera_layout_system,
    update_camera_view_buffer_system, CentralCameraViewBindGroupLayout, CentralCameraViewBuffer,
};
