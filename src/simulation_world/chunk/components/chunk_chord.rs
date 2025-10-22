use crate::simulation_world::chunk::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use glam::IVec3;

pub const X_SHIFT: i32 = CHUNK_WIDTH.trailing_zeros() as i32;
pub const Y_SHIFT: i32 = CHUNK_HEIGHT.trailing_zeros() as i32;
pub const Z_SHIFT: i32 = CHUNK_DEPTH.trailing_zeros() as i32;

/// Stores the coordinate of the chunk an entity is currently in.
#[derive(Component, Deref, DerefMut, Debug)]
pub struct ChunkChord {
    pub pos: IVec3,
}

/// Convert a world position to chunk coordinate
pub fn world_to_chunk_pos(world_pos: glam::Vec3) -> IVec3 {
    let int_pos = world_pos.floor().as_ivec3();

    IVec3::new(
        int_pos.x >> X_SHIFT,
        int_pos.y >> Y_SHIFT,
        int_pos.z >> Z_SHIFT,
    )
}
