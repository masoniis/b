use crate::prelude::*;
use crate::simulation_world::block::SOLID_BLOCK_ID;
use crate::simulation_world::chunk::{ChunkCoord, VolumeDataWriter};
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    block::{BlockId, BlockRegistryResource},
    chunk::{
        types::ChunkLod, ChunkBlocksComponent, WorldVoxelIteratorWithColumn,
        WorldVoxelPositionIterator,
    },
    terrain::{
        BiomeMapComponent, ClimateNoiseGenerator, DefaultBiomeGenerator, SimpleSurfacePainter,
        SuperflatShaper, TerrainClimateMapComponent,
    },
};
use bevy_ecs::prelude::{Component, Resource};
use std::{fmt::Debug, sync::Arc};

/// A resource holding the active terrain chunk generator.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct ActiveTerrainGenerator(pub Arc<dyn TerrainShaper>);

impl Default for ActiveTerrainGenerator {
    fn default() -> Self {
        ActiveTerrainGenerator(Arc::new(SuperflatShaper::new()))
    }
}

/// A resource holding the active biome chunk generator.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct ActiveBiomeGenerator(pub Arc<dyn BiomeGenerator>);

impl Default for ActiveBiomeGenerator {
    fn default() -> Self {
        Self(Arc::new(DefaultBiomeGenerator::default()))
    }
}

/// A resource holding the active terrain chunk painter.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct ActiveTerrainPainter(pub Arc<dyn TerrainPainter>);

impl Default for ActiveTerrainPainter {
    fn default() -> Self {
        Self(Arc::new(SimpleSurfacePainter::new()))
    }
}

// INFO: -------------------------
//         Biome generator
// -------------------------------

/// A trait for just filling the biome map
pub trait BiomeGenerator: Send + Sync + Debug {
    fn generate_biome_chunk(
        &self,
        biome_map: &mut BiomeMapComponent,
        terrain_climate_map: &mut TerrainClimateMapComponent,
        iterator: WorldVoxelIteratorWithColumn,

        climate_noise: &ClimateNoiseGenerator,
        biome_registry: &BiomeRegistryResource,
    );
}

/// A struct representing generated biome data for every block in a chunk.
pub struct GeneratedBiomeData {
    pub biome_map: BiomeMapComponent,
    pub terrain_climate_map: TerrainClimateMapComponent,
}

impl GeneratedBiomeData {
    pub fn empty(lod: ChunkLod) -> Self {
        Self {
            biome_map: BiomeMapComponent::new_empty(lod),
            terrain_climate_map: TerrainClimateMapComponent::new_empty(lod),
        }
    }

    pub fn as_tuple(self) -> (BiomeMapComponent, TerrainClimateMapComponent) {
        (self.biome_map, self.terrain_climate_map)
    }
}

// INFO: ------------------------
//         terrain shaper
// ------------------------------

/// A trait for chunk shapers to implement.
pub trait TerrainShaper: Send + Sync + Debug {
    /// Takes in empty chunk blocks and fills them in according to the generator's logic.
    fn shape_terrain_chunk(
        &self,
        // input
        climate_map: &TerrainClimateMapComponent,

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

// INFO: -------------------------
//         terrain painter
// -------------------------------

pub trait TerrainPainter: Send + Sync + Debug {
    fn paint_terrain_chunk(
        &self,
        painter: PaintResultBuilder,
        iterator: WorldVoxelPositionIterator,

        biome_map: &BiomeMapComponent,
        climate_map: &TerrainClimateMapComponent,

        block_registry: &BlockRegistryResource,
        biome_registry: &BiomeRegistryResource,
    ) -> PaintResultBuilder;
}

pub struct PaintResultBuilder {
    blocks: ChunkBlocksComponent,
    pub chunk_coord: ChunkCoord,
    metadata: ChunkMetadata,
    block_registry: BlockRegistryResource,
}

impl PaintResultBuilder {
    /// Creates a new painter, taking ownership of the ChunkBlocksComponent.
    pub fn new(
        blocks: ChunkBlocksComponent,
        chunk_coord: ChunkCoord,
        block_registry: BlockRegistryResource,
    ) -> Self {
        Self {
            blocks,
            chunk_coord,
            metadata: ChunkMetadata::new(),
            block_registry,
        }
    }

    /// Returns the size of the chunk.
    pub fn size(&self) -> usize {
        self.blocks.size()
    }

    /// Returns a read-only view (useful for early exit checks).
    pub fn is_uniform(&self) -> Option<BlockId> {
        self.blocks.is_uniform()
    }

    /// Opens a high-performance edit scope.
    #[inline(always)]
    pub fn edit_arbitrary(&mut self, mut f: impl FnMut(&mut PaintWriter)) {
        let block_writer = self.blocks.get_writer();

        let mut writer = PaintWriter {
            block_writer,
            metadata: &mut self.metadata,
            registry: &self.block_registry,
        };

        f(&mut writer);
    }

    /// Runs an optimally structured loop (X, Z, Y) for painting logic.
    ///
    /// The closure receives:
    /// 1. Local Coordinate (x, y, z)
    /// 2. World Coordinate (wx, wy, wz)
    /// 3. Current Block ID
    ///
    /// It should return `Some(NewBlockId)` to change the block, or `None` to leave it alone.
    #[inline(always)]
    pub fn fill_from(&mut self, f: impl Fn(IVec3, IVec3, BlockId) -> Option<BlockId>) {
        let size = self.blocks.size() as i32;
        let lod = self.blocks.lod();

        let base_world = self.chunk_coord.as_world_pos();
        let step = 1 << lod.0;

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

                        // call closure with args, updating if relevant
                        let current_block = writer.get_block(x as usize, y as usize, z as usize);
                        if let Some(new_block) = f(local, world, current_block) {
                            writer.set_block(x as usize, y as usize, z as usize, new_block);
                        }
                    }
                }
            }
        });
    }

    /// Consumes the builder and returns the final generated components.
    pub fn finish(self) -> (ChunkBlocksComponent, ChunkMetadata) {
        (self.blocks, self.metadata)
    }
}

/// A temporary helper for writing blocks and updating metadata efficiently.
pub struct PaintWriter<'a> {
    block_writer: VolumeDataWriter<'a, BlockId>,
    metadata: &'a mut ChunkMetadata,
    registry: &'a BlockRegistryResource,
}

impl<'a> PaintWriter<'a> {
    /// Sets a block in the chunk and updates metadata (uniformity, transparency).
    #[inline(always)]
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: BlockId) {
        self.block_writer.set_data(x, y, z, block_id);
        self.update_metadata(block_id);
    }

    /// Fills the entire chunk with a single block efficiently.
    #[inline(always)]
    pub fn fill(&mut self, block_id: BlockId) {
        self.block_writer.fill(block_id)
    }

    /// Gets a block from the chunk.
    #[inline(always)]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockId {
        self.block_writer.get_data(x, y, z)
    }

    /// Helper to get block properties via registry if needed for logic decisions
    #[inline(always)]
    pub fn get_block_properties(
        &self,
        block_id: BlockId,
    ) -> &crate::simulation_world::block::BlockProperties {
        self.registry.get(block_id)
    }

    #[inline(always)]
    fn update_metadata(&mut self, block_id: BlockId) {
        // uniformity
        if self.metadata.is_uniform {
            if let Some(first) = self.metadata.uniform_block_id {
                if first != block_id {
                    self.metadata.is_uniform = false;
                    self.metadata.uniform_block_id = None;
                }
            } else {
                self.metadata.uniform_block_id = Some(block_id);
            }
        }

        // transparency
        if !self.metadata.contains_transparent {
            let props = self.registry.get(block_id);
            if props.is_transparent {
                self.metadata.contains_transparent = true;
            }
        }
    }
}

/// Contains all metadata calculated during generation.
#[derive(Component, Debug, Clone)]
pub struct ChunkMetadata {
    /// If true, all blocks in the chunk are identical.
    pub is_uniform: bool,
    /// If uniform, this is the ID. If mixed, this is None.
    /// Note: Used for optimization hints.
    pub uniform_block_id: Option<BlockId>,
    /// If true, the chunk contains at least one transparent block.
    pub contains_transparent: bool,
}

/// A struct to track metadata state during generation.
impl ChunkMetadata {
    pub fn new() -> Self {
        Self {
            is_uniform: true,
            uniform_block_id: None,
            contains_transparent: false,
        }
    }
}

// INFO: -----------------------
//         bundled types
// -----------------------------

pub struct GeneratedChunkComponentBundle {
    pub chunk_blocks: Option<ChunkBlocksComponent>,
    pub chunk_metadata: Option<ChunkMetadata>,
    pub biome_map: BiomeMapComponent,
}
