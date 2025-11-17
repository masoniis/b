use crate::prelude::*;
use crate::simulation_world::terrain::climate::ClimateMapComponent;
use crate::simulation_world::{
    block::{BlockId, SOLID_BLOCK_ID},
    chunk::{ChunkBlocksComponent, ChunkCoord, VolumeDataWriter},
};
use std::fmt::Debug;

// INFO: ------------------------
//         terrain shaper
// ------------------------------

/// A trait for chunk shapers to implement.
pub trait TerrainShaper: Send + Sync + Debug {
    /// Takes in empty chunk blocks and fills them in according to the generator's logic.
    fn shape_terrain_chunk(
        &self,
        // input
        climate_map: &ClimateMapComponent,

        // output
        shaper: ShapeResultBuilder,
    ) -> ShapeResultBuilder;

    /// A fast, cheap check to see if this chunk will be uniform (all air or all solid).
    ///
    /// By implementing this, generators can help the engine optimize performance with
    /// the ability to entirely skip generating uniform chunks, and additionally skip
    /// biome compute for all air (empty) chunks.
    fn determine_chunk_uniformity(&self, _: IVec3) -> ChunkUniformity {
        ChunkUniformity::Mixed
    }
}

/// Describes the density uniformity of a chunk.
#[derive(Debug, PartialEq, Eq)]
pub enum ChunkUniformity {
    /// The chunk is 100% empty (all air).
    Empty,
    /// The chunk is 100% solid (all filled).
    Solid,
    /// The chunk contains a mix of empty and solid blocks.
    Mixed,
}

/// A writer for updating terrain shape data.
pub struct ShapeWriter<'a> {
    block_writer: VolumeDataWriter<'a, BlockId>,
    // could add some other logic like the painter eventually
}

impl<'a> ShapeWriter<'a> {
    #[inline(always)]
    pub fn mark_solid(&mut self, x: usize, y: usize, z: usize) {
        self.block_writer.set_data(x, y, z, SOLID_BLOCK_ID);
    }
}

pub struct ShapeResultBuilder {
    blocks: ChunkBlocksComponent,
    chunk_coord: ChunkCoord,
}

impl ShapeResultBuilder {
    pub fn new(blocks: ChunkBlocksComponent, chunk_coord: ChunkCoord) -> Self {
        Self {
            blocks,
            chunk_coord,
        }
    }

    /// Finish shaping and take ownership of the inner blocks component.
    pub fn finish(self) -> ChunkBlocksComponent {
        self.blocks
    }

    /// Opens a manual edit scope for arbitrary writes.
    ///
    /// WARNING: Caller is responsible for loop ordering and coordinate math.
    /// Incorrect usage may break auto-vectorization.
    #[inline(always)]
    pub fn edit_arbitrary(&mut self, mut f: impl FnMut(&mut ShapeWriter)) {
        let block_writer = self.blocks.get_writer();
        let mut writer = ShapeWriter { block_writer };
        f(&mut writer);
    }

    /// Runs an optimally structured loop (X, Z, Y) to fill blocks based on the closure.
    ///
    /// The closure should return `true` for solid blocks and `false` for air blocks.
    #[inline(always)]
    pub fn fill_from(&mut self, f: impl Fn(IVec3, IVec3) -> bool) {
        let size = self.blocks.size() as i32;
        let base_world = self.chunk_coord.as_world_pos();
        let step = 1 << self.blocks.lod().0;

        self.edit_arbitrary(|writer| {
            let base_x = base_world.x;
            let base_y = base_world.y;
            let base_z = base_world.z;

            for x in 0..size {
                let world_x = base_x + (x * step);
                for z in 0..size {
                    let world_z = base_z + (z * step);
                    for y in 0..size {
                        let world_y = base_y + (y * step);

                        let local = IVec3::new(x, y, z);
                        let world = IVec3::new(world_x, world_y, world_z);

                        if f(local, world) {
                            writer.mark_solid(x as usize, y as usize, z as usize);
                        }
                    }
                }
            }
        });
    }
}
