use bevy_ecs::prelude::Component;

use crate::simulation_world::chunk::CHUNK_AREA;

/// Heightmap of the highest non-transparent block.
#[derive(Component, Clone)]
pub struct SurfaceHeightmap(pub [u16; CHUNK_AREA]);

impl SurfaceHeightmap {
    /// Creates a new empty surface heightmap.
    pub fn empty() -> Self {
        Self([0; CHUNK_AREA])
    }
}

/// Heightmap of the highest solid block.
#[derive(Component, Clone)]
pub struct WorldSurfaceHeightmap(pub [u16; CHUNK_AREA]);

impl WorldSurfaceHeightmap {
    /// Creates a new empty surface heightmap.
    pub fn empty() -> Self {
        Self([0; CHUNK_AREA])
    }
}
