use crate::prelude::*;
use crate::simulation_world::{
    block::block_registry::AIR_BLOCK_ID,
    chunk::{
        components::{ChunkBlocksComponent, ChunkCoord, ChunkMeshDirty},
        CHUNK_SIDE_LENGTH,
    },
};
use bevy_ecs::prelude::{Commands, Entity, Message, MessageReader, Query};

/// An event that is sent when a voxel should be broken.
#[derive(Message, Clone)]
pub struct BreakVoxelEvent {
    /// The world position of the voxel to break.
    pub world_pos: IVec3,
}

/// A system that handles the `BreakVoxelEvent`.
pub fn break_voxel_system(
    // input
    mut events: MessageReader<BreakVoxelEvent>,

    // output
    mut chunks: Query<(Entity, &ChunkCoord, &mut ChunkBlocksComponent)>,
    mut commands: Commands,
) {
    for event in events.read() {
        let chunk_pos = ChunkCoord::world_to_chunk_pos(event.world_pos.as_vec3());

        if let Some((entity, _, mut chunk_blocks)) = chunks
            .iter_mut()
            .find(|(_, coord, _)| coord.pos == chunk_pos)
        {
            let local_pos = event.world_pos - (chunk_pos * CHUNK_SIDE_LENGTH as i32);

            chunk_blocks.set_data(
                local_pos.x as usize,
                local_pos.y as usize,
                local_pos.z as usize,
                AIR_BLOCK_ID,
            );

            commands.entity(entity).insert(ChunkMeshDirty);
        }
    }
}
