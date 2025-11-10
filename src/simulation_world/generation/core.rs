use crate::prelude::*;
use crate::simulation_world::block::BlockRegistryResource;
use crate::simulation_world::chunk::{WorldVoxelIteratorWithColumn, WorldVoxelPositionIterator};
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    chunk::{types::ChunkLod, ChunkBlocksComponent},
    generation::{
        BiomeMapComponent, ClimateNoiseGenerator, DefaultBiomeGenerator, SuperflatGenerator,
        TerrainClimateMapComponent,
    },
};
use bevy_ecs::prelude::Resource;
use std::{fmt::Debug, sync::Arc};

/// A resource holding the active terrain chunk generator.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct ActiveTerrainGenerator(pub Arc<dyn TerrainGenerator>);

impl Default for ActiveTerrainGenerator {
    fn default() -> Self {
        ActiveTerrainGenerator(Arc::new(SuperflatGenerator::new()))
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

/// A trait for chunk generators to implement.
pub trait TerrainGenerator: Send + Sync + Debug {
    /// Takes in empty chunk blocks and fills them in according to the generator's logic.
    fn generate_terrain_chunk(
        &self,
        chunk_blocks: &mut ChunkBlocksComponent,
        iterator: WorldVoxelPositionIterator,

        biome_map: &BiomeMapComponent,
        climate_map: &TerrainClimateMapComponent,

        block_registry: &BlockRegistryResource,
        biome_registry: &BiomeRegistryResource,
    );

    /// A fast, cheap check to see if this chunk will be *guaranteed* empty.
    /// If this returns `true`, `generate_terrain_chunk` is never called and
    /// the biome generation never happens, resulting in massive perf gains.
    ///
    /// This is an optimization specific to each generator. By default, we
    /// assume the chunk is not empty to force the full generation path.
    fn is_chunk_empty(&self, _: IVec3) -> bool {
        false
    }
}

/// A struct representing generated chunk data.
pub struct GeneratedTerrainData {
    pub chunk_blocks: Option<ChunkBlocksComponent>,
}

impl GeneratedTerrainData {
    /// Generates an empty chunk data instance.
    pub fn empty() -> Self {
        Self { chunk_blocks: None }
    }

    /// Generates an all-air chunk data instance.
    pub fn all_air() -> Self {
        Self {
            chunk_blocks: Some(ChunkBlocksComponent::new_empty(ChunkLod(0))),
        }
    }
}

// INFO: -----------------------
//         Bundled types
// -----------------------------

pub struct GeneratedChunkComponentBundle {
    pub chunk_blocks: Option<ChunkBlocksComponent>,
    pub biome_map: BiomeMapComponent,
}

/// A resource holding a list of terrain generators to cycle through.
#[derive(Resource)]
pub struct TerrainGeneratorCycle {
    pub generators: Vec<Arc<dyn TerrainGenerator>>,
    pub current_index: usize,
}

impl Default for TerrainGeneratorCycle {
    fn default() -> Self {
        Self {
            generators: Vec::new(),
            current_index: 0,
        }
    }
}