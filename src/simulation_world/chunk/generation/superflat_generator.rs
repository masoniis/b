use crate::prelude::*;
use crate::simulation_world::block::property_registry::BlockRegistryResource;
use crate::simulation_world::chunk::chunk::ChunkComponent;
use crate::simulation_world::chunk::{
    ChunkGenerator, GeneratedChunkComponents, TransformComponent, CHUNK_DEPTH, CHUNK_HEIGHT,
    CHUNK_WIDTH,
};
use glam::{IVec3, Quat};

#[derive(Debug, Clone)]
pub struct SuperflatGenerator {
    layers: Vec<String>,
}

impl SuperflatGenerator {
    pub fn new() -> Self {
        Self {
            layers: vec![
                "stone".to_string(),
                "stone".to_string(),
                "grass".to_string(),
            ],
        }
    }
}

impl ChunkGenerator for SuperflatGenerator {
    fn generate_chunk(
        &self,
        coord: IVec3,
        blocks: &BlockRegistryResource,
    ) -> GeneratedChunkComponents {
        let cx = coord.x;
        let cy = coord.y;
        let cz = coord.z;

        if cy != 0 {
            return GeneratedChunkComponents::empty();
        }

        let layer_blocks: Vec<_> = self
            .layers
            .iter()
            .map(|name| blocks.get_block_by_name(name))
            .collect();

        let mut chunk = ChunkComponent::new();

        for x in 1..CHUNK_WIDTH - 1 {
            for z in 1..CHUNK_DEPTH - 1 {
                for (y, block) in layer_blocks.iter().enumerate() {
                    if y < CHUNK_HEIGHT {
                        chunk.set_block(x, y, z, *block);
                    }
                }
            }
        }

        let transform = TransformComponent {
            position: Vec3::new(
                (cx * CHUNK_WIDTH as i32) as f32,
                (cy * CHUNK_HEIGHT as i32) as f32,
                (cz * CHUNK_DEPTH as i32) as f32,
            ),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        };

        GeneratedChunkComponents {
            chunk_component: chunk,
            transform_component: transform,
        }
    }
}
