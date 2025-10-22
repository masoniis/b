use crate::render_world::textures::GpuTextureArray;
use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct TextureArrayResource {
    pub array: GpuTextureArray,
}
