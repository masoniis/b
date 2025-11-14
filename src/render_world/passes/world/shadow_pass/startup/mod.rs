pub mod setup_shadow_pipeline;
pub mod setup_shadow_texture;
pub mod setup_view_buffer;

pub use setup_shadow_pipeline::{setup_shadow_pass_pipeline, ShadowPassPipeline};
pub use setup_shadow_texture::{
    setup_shadow_depth_texture_system, ShadowDepthTextureResource, SHADOW_DEPTH_FORMAT,
    SHADOW_MAP_RESOLUTION,
};
pub use setup_view_buffer::{setup_shadow_view_buffer_system, ShadowViewBuffer, ShadowViewData};
