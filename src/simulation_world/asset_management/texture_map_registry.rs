use crate::render_world::textures::TextureRegistry;
use bevy_ecs::prelude::Resource;

/// A resource holding the texture registry for looking up texture indices by name.
///
/// Cheaply clonable due to internal Arc usage.
#[derive(Resource, Clone)]
pub struct TextureMapResource {
    pub registry: TextureRegistry,
}
