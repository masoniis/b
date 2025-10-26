use crate::simulation_world::{biome::biome_registry::BiomeId, chunk::CHUNK_AREA};
use bevy_ecs::component::Component;

#[derive(Component)]
pub struct BiomeMap(pub [BiomeId; CHUNK_AREA]);

impl BiomeMap {
    pub fn empty() -> Self {
        Self([BiomeId::default(); CHUNK_AREA])
    }
}
