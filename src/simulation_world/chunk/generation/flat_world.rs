use crate::prelude::*;
use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::superflat_generator::SuperflatGenerator;
use crate::simulation_world::chunk::ChunkGenerator;
use bevy_ecs::prelude::*;
use glam::IVec3;

/// System to spawn the initial chunk entities for a flat world
#[instrument(skip_all)]
pub fn setup_superflat_world(mut commands: Commands, blocks: Res<BlockRegistryResource>) {
    let generator = SuperflatGenerator::new();

    for cx in -12..12 {
        for cz in -12..12 {
            let generated_chunk = generator.generate_chunk(IVec3::new(cx, 0, cz), &blocks);
            commands.spawn((generated_chunk.chunk, generated_chunk.transform));
        }
    }

    info!("Superflat world generation complete!");
}
