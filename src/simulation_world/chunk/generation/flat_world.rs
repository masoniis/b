use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::chunk::Chunk;
use crate::simulation_world::chunk::components::TransformComponent;
use crate::simulation_world::chunk::CHUNK_DEPTH;
use crate::{prelude::*, simulation_world::chunk::CHUNK_WIDTH};
use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};

/// Defines the layers of blocks in the superflat world, from bottom to top
const LAYERS: &[&str] = &["stone", "stone", "grass"];

/// System to spawn the initial chunk entities for a flat world
#[instrument(skip_all)]
pub fn setup_superflat_world(mut commands: Commands, blocks: Res<BlockRegistryResource>) {
    info!("Generating superflat world with {} layers...", LAYERS.len());

    // Pre-fetch all block types we need
    let layer_blocks: Vec<_> = LAYERS
        .iter()
        .map(|name| blocks.get_block_by_name(name))
        .collect();

    for cx in -12..12 {
        for cz in -12..12 {
            let mut chunk = Chunk::new(cx, 0, cz);

            for x in 0..CHUNK_WIDTH {
                for z in 0..CHUNK_DEPTH {
                    for (y, block) in layer_blocks.iter().enumerate() {
                        chunk.set_block(x, y, z, *block);
                    }
                }
            }
            chunk.set_block(3, 3, 3, blocks.get_block_by_name("dirt"));
            chunk.set_block(3, 4, 3, blocks.get_block_by_name("dirt"));
            chunk.set_block(3, 5, 3, blocks.get_block_by_name("dirt"));

            commands.spawn((
                chunk,
                TransformComponent {
                    position: Vec3::new(
                        (cx * CHUNK_WIDTH as i32) as f32,
                        0.0,
                        (cz * CHUNK_DEPTH as i32) as f32,
                    ),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
            ));
        }
    }

    info!("Superflat world generation complete!");
}
