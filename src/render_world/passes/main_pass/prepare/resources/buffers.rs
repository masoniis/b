use bevy_ecs::prelude::Resource;
use wgpu::{Texture, TextureView};

pub const SHADER_PATH: &str = "assets/shaders/scene/simple.wesl";
pub const LOADING_SHADER_PATH: &str = "assets/shaders/loading_screen/loading.wesl";
pub const MAX_TRANSFORMS: u64 = 100000;

/// A resource to hold the depth texture and its view, used for depth testing.
#[derive(Resource)]
pub struct DepthTextureResource {
    /// The depth texture itself. We must hold this to prevent it from being dropped.
    pub texture: Texture,
    /// The view of the depth texture, which is what render passes use.
    pub view: TextureView,
}
