use crate::graphics::textures::Texture;
use std::collections::HashMap;

#[derive(Default)]
pub struct TextureManager {
    atlases: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn add_atlas(&mut self, id: String, texture: Texture) {
        self.atlases.insert(id, texture);
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture> {
        self.atlases.get(id)
    }
}
