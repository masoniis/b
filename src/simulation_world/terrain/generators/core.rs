use crate::prelude::*;
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    block::{BlockId, BlockRegistryResource, SOLID_BLOCK_ID},
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

// INFO: ---------------------------
//         Terrain generator
// ---------------------------------

/// A trait for chunk shapers to implement.
pub trait TerrainShaper: Send + Sync + Debug {
    /// Takes in empty chunk blocks and fills them in according to the generator's logic.
    fn shape_terrain_chunk(
        &self,
        // input
        iterator: WorldVoxelPositionIterator,
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

pub struct ShapeResultBuilder {
    blocks: ChunkBlocksComponent,
    // TODO: add is all solid here probably
}

impl ShapeResultBuilder {
    pub fn new(blocks: ChunkBlocksComponent) -> Self {
        Self { blocks }
    }

    pub fn mark_as_solid(&mut self, x: usize, y: usize, z: usize) {
        self.blocks.set_data(x, y, z, SOLID_BLOCK_ID);
    }

    pub fn finish(self) -> ChunkBlocksComponent {
        self.blocks
    }
}

// INFO: -------------------------
//         Terrain painter
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

/// The final component to be stored on the chunk entity.
/// Contains all metadata calculated during generation.
#[derive(Component, Debug)]
pub struct ChunkMetadata {
    pub is_uniform: bool,
    /// The block ID of the chunk, if it's uniform.
    /// `None` implies a uniform, empty (all-air) chunk.
    pub uniform_block_id: Option<BlockId>,
    pub contains_transparent: bool,
    /// 2D heightmap of the highest non-air block in each (x, z) column.
    pub height_map: Vec<Vec<usize>>,
    // You could add other metadata here later, like:
    // pub contains_tickable_blocks: bool,
}

impl ChunkMetadata {
    fn new(size: usize) -> Self {
        Self {
            is_uniform: true,
            uniform_block_id: None,
            contains_transparent: false,
            // Initialize the height_map with the correct dimensions
            height_map: vec![vec![0; size]; size],
        }
    }
}

/// A struct to track metadata state during generation.
///
/// Can be built to extract components for world insertion.
pub struct PaintResultBuilder {
    blocks: ChunkBlocksComponent,
    metadata: ChunkMetadata,

    block_registry: BlockRegistryResource,
}

impl PaintResultBuilder {
    /// Creates a new builder, taking ownership of the (likely empty)
    /// ChunkBlocksComponent.
    pub fn new(blocks: ChunkBlocksComponent, block_registry: BlockRegistryResource) -> Self {
        let size = blocks.size(); // for heightmap
        let metadata = ChunkMetadata::new(size);
        Self {
            blocks,
            metadata,
            block_registry,
        }
    }

    /// Sets a block in the chunk and updates the metadata accordingly.
    #[inline(always)]
    pub fn set_data(&mut self, x: usize, y: usize, z: usize, block_id: BlockId) {
        self.blocks.set_data(x, y, z, block_id);
        self.update_metadata_state(x, y, z, block_id);
    }

    /// Gets the block ID at the given local coordinates without bounds checking.
    #[inline(always)]
    pub fn get_data_unchecked(&self, x: usize, y: usize, z: usize) -> BlockId {
        self.blocks.get_data_unchecked(x, y, z)
    }

    pub fn get_chunk_size(&self) -> usize {
        self.blocks.size()
    }

    /// The internal metadata update, which can now use the registry.
    #[inline(always)]
    fn update_metadata_state(&mut self, x: usize, y: usize, z: usize, block_id: BlockId) {
        // uniformity check
        if self.metadata.is_uniform {
            if let Some(first_id) = self.metadata.uniform_block_id {
                if first_id != block_id {
                    self.metadata.is_uniform = false;
                }
            } else {
                self.metadata.uniform_block_id = Some(block_id);
            }
        }

        // transparency check
        if !self.metadata.contains_transparent {
            let props = self.block_registry.get(block_id);
            if props.is_transparent {
                self.metadata.contains_transparent = true;
            }
        }

        // heightmap update
        let props = self.block_registry.get(block_id);
        if !(props.display_name == "Air".to_string()) {
            let current_max_y = &mut self.metadata.height_map[x][z];
            if y > *current_max_y {
                *current_max_y = y;
            }
        }
    }

    /// Consumes the builder and returns the final generated components,
    /// passing ownership of the blocks and the new metadata.
    pub fn finish(self) -> (ChunkBlocksComponent, ChunkMetadata) {
        (self.blocks, self.metadata)
    }
}

// INFO: -----------------------
//         Bundled types
// -----------------------------

pub struct GeneratedChunkComponentBundle {
    pub chunk_blocks: Option<ChunkBlocksComponent>,
    pub chunk_metadata: Option<ChunkMetadata>,
    pub biome_map: BiomeMapComponent,
}
