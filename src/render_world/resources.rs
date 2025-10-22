use crate::render_world::textures::GpuTextureArray;
use bevy_ecs::prelude::Resource;
use std::collections::HashMap;
use wgpu::RenderPipeline;

#[derive(Resource)]
pub struct TextureArrayResource {
    pub array: GpuTextureArray,
}
