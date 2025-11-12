use crate::prelude::*;
use crate::simulation_world::block::TargetedBlock;
use crate::simulation_world::{
    block::block_registry::SOLID_BLOCK_ID,
    chunk::{
        components::{ChunkBlocksComponent, ChunkCoord, ChunkMeshDirty},
        CHUNK_SIDE_LENGTH,
    },
};
use bevy_ecs::prelude::{Commands, Entity, Message, MessageReader, Query};
use bevy_ecs::prelude::{MessageWriter, Res};

/// An event that is sent when a voxel should be placed.
#[derive(Message, Clone)]
pub struct PlaceVoxelEvent {
    /// The world position of the block *face* that was targeted.
    /// The new block will be placed adjacent to this.
    pub target_pos: IVec3,
}

/// Fires a `PlaceVoxelEvent` for the currently targeted block.
pub fn place_targeted_voxel_system(
    targeted_block: Res<TargetedBlock>,
    mut place_voxel_writer: MessageWriter<PlaceVoxelEvent>,
) {
    if let Some(voxel_pos) = targeted_block.position {
        place_voxel_writer.write(PlaceVoxelEvent {
            target_pos: voxel_pos,
        });
    }
}

/// A system that handles the `PlaceVoxelEvent`.
pub fn handle_place_voxel_events_system(
    // input
    mut events: MessageReader<PlaceVoxelEvent>,

    // output
    mut chunks: Query<(Entity, &ChunkCoord, &mut ChunkBlocksComponent)>,
    mut commands: Commands,
) {
    for event in events.read() {
        // TODO: properly place against face highlighted, not just y + 1
        let new_block_pos = event.target_pos + IVec3::Y;

        let chunk_pos = ChunkCoord::world_to_chunk_pos(new_block_pos.as_vec3());

        if let Some((entity, _, mut chunk_blocks)) = chunks
            .iter_mut()
            .find(|(_, coord, _)| coord.pos == chunk_pos)
        {
            let local_pos = new_block_pos - (chunk_pos * CHUNK_SIDE_LENGTH as i32);

            chunk_blocks.set_data(
                local_pos.x as usize,
                local_pos.y as usize,
                local_pos.z as usize,
                SOLID_BLOCK_ID,
            );

            commands.entity(entity).insert(ChunkMeshDirty);
        }
    }
}
