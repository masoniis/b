use crate::{render_world::context::GraphicsContext, render_world::textures::TextureArray};
use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct GraphicsContextResource {
    pub context: GraphicsContext,
}

#[derive(Resource)]
pub struct TextureArrayResource {
    pub array: TextureArray,
}
