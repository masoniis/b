use crate::render_world::textures::TextureRegistry;
use bevy_ecs::prelude::Resource;

/// A resource holding the texture registry for looking up texture indices by name.
#[derive(Resource)]
pub struct TextureMapResource {
    pub registry: TextureRegistry,
}
