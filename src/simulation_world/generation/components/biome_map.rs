use crate::simulation_world::{
    biome::biome_registry::BiomeId,
    chunk::{CHUNK_SIZE, Y_SHIFT, Z_SHIFT},
};
use bevy_ecs::component::Component;

/// The biome map which stores the biome ID for every block in a chunk.
#[derive(Component, Debug)]
pub struct BiomeMapComponent(pub [BiomeId; CHUNK_SIZE]);

impl BiomeMapComponent {
    pub fn empty() -> Self {
        Self([BiomeId::default(); CHUNK_SIZE])
    }

    #[inline(always)]
    pub fn get_biome(&self, x: usize, y: usize, z: usize) -> BiomeId {
        let index = (y << Y_SHIFT) | (z << Z_SHIFT) | x;
        self.0[index]
    }

    #[inline(always)]
    pub fn set_biome(&mut self, x: usize, y: usize, z: usize, biome: BiomeId) {
        let index = (y << Y_SHIFT) | (z << Z_SHIFT) | x;
        self.0[index] = biome;
    }
}
