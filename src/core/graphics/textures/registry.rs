use std::collections::HashMap;

/// A registry for looking up texture indices by name.
pub struct TextureRegistry {
    map: HashMap<String, u32>,
    missing_texture_index: u32,
}

impl TextureRegistry {
    /// Creates a new texture registry from a name-to-index map.
    pub fn new(map: HashMap<String, u32>, missing_texture_index: u32) -> Self {
        Self {
            map,
            missing_texture_index,
        }
    }

    /// Gets the texture index for a given name.
    pub fn get(&self, name: &str) -> Option<u32> {
        self.map.get(name).copied()
    }

    /// Gets the texture index for a given name, panicking if not found.
    /// Use this when you're certain the texture exists.
    pub fn get_unchecked(&self, name: &str) -> u32 {
        self.map[name]
    }

    /// Gets the texture index for a given name, returning the missing texture if not found.
    /// This is the recommended method for safe texture lookups.
    pub fn get_or_missing(&self, name: &str) -> u32 {
        self.get(name).unwrap_or(self.missing_texture_index)
    }

    /// Gets the texture index for a given name, returning a default if not found.
    pub fn get_or_default(&self, name: &str, default: u32) -> u32 {
        self.get(name).unwrap_or(default)
    }

    /// Returns the missing texture index.
    pub fn missing_texture(&self) -> u32 {
        self.missing_texture_index
    }

    /// Returns true if the registry contains a texture with the given name.
    pub fn contains(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    /// Returns the number of textures in the registry (including the missing texture).
    pub fn len(&self) -> usize {
        self.map.len() + 1 // +1 for missing texture
    }

    /// Returns true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
