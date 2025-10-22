use crate::simulation_world::chunk::CHUNK_WIDTH;
use bevy_ecs::prelude::*;
use glam::IVec3;

/// Stores the coordinate of the chunk an entity is currently in.
#[derive(Component)]
pub struct ChunkVicinity {
    pub pos: IVec3,
}

/// Convert a world position to chunk coordinate
pub fn world_to_chunk_pos(world_pos: glam::Vec3) -> IVec3 {
    (world_pos / CHUNK_WIDTH as f32).floor().as_ivec3()
}
