use crate::prelude::*;
use crate::simulation_world::biome::biome_registry::BiomeId;
use crate::simulation_world::chunk::types::{ChunkLod, ChunkVolumeData};
use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Deref, DerefMut)]
pub struct BiomeMapComponent(pub ChunkVolumeData<BiomeId>);

impl BiomeMapComponent {
    pub fn new_empty(lod: ChunkLod) -> Self {
        Self(ChunkVolumeData::new_filled(lod, 0))
    }
}
