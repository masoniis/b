use crate::render_world::textures::TextureLoadError;

use super::super::types::TextureId;
use std::collections::HashMap;

/// A registry for looking up texture indices from a compile-time safe TextureId.
pub struct TextureRegistry {
    /// Maps the type-safe TextureId to its u32 index in the GPU texture array.
    map: HashMap<TextureId, u32>,

    /// The index of the fallback "missing texture" pattern.
    missing_texture_index: u32,
}

impl TextureRegistry {
    /// Creates a new texture registry from a pre-populated map and the missing texture index.
    pub fn new(map: HashMap<TextureId, u32>) -> Result<Self, TextureLoadError> {
        let missing_texture_index = *map
            .get(&TextureId::Missing)
            .ok_or(TextureLoadError::MissingTextureNotInManifest)?;

        Ok(Self {
            map,
            missing_texture_index,
        })
    }

    /// Gets the texture index for a given ID, panicking if not found.
    pub fn get(&self, id: TextureId) -> u32 {
        self.map[&id]
    }

    /// Returns the missing texture index.
    pub fn missing_texture(&self) -> u32 {
        self.missing_texture_index
    }

    /// Returns true if the registry contains a texture with the given ID.
    pub fn contains(&self, id: TextureId) -> bool {
        self.map.contains_key(&id)
    }

    /// Returns the total number of textures in the registry.
    /// This assumes the "missing texture" is an entry within the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }
}
