use crate::ecs_modules::world::utils::types::chunk::Chunk;
use crate::ecs_modules::world::world_gen::{generate_flat_world_chunk, CHUNK_HEIGHT};
use bevy_ecs::prelude::Commands;
use tracing::info;

pub fn chunk_generation_system(_commands: Commands) {
    info!("Generating initial chunk...");

    let mut chunk = Chunk::new(0, 0, 0);
    generate_flat_world_chunk(&mut chunk);

    // Temporary verification: print some block IDs
    info!(
        "Block at (0, 0, 0): {:?}",
        chunk.get_block(0, 0, 0).map(|b| b.id)
    );
    info!(
        "Block at (0, {}, 0): {:?}",
        CHUNK_HEIGHT - 2,
        chunk.get_block(0, CHUNK_HEIGHT - 2, 0).map(|b| b.id)
    );
    info!(
        "Block at (0, {}, 0): {:?}",
        CHUNK_HEIGHT - 1,
        chunk.get_block(0, CHUNK_HEIGHT - 1, 0).map(|b| b.id)
    );
}
