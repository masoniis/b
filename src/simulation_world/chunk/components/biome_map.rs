use crate::simulation_world::chunk::{CHUNK_SIZE, CHUNK_SURFACE_SIZE};
use bevy_ecs::component::Component;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BiomeId {
    #[default]
    Ocean = 0,
    Plains = 1,
    Desert = 2,
    Forest = 3,
    Mountains = 4,
}

#[derive(Component)]
pub struct BiomeMap(pub [BiomeId; CHUNK_SURFACE_SIZE]);

impl BiomeMap {
    pub fn empty() -> Self {
        Self([BiomeId::default(); CHUNK_SURFACE_SIZE])
    }
}
