use crate::prelude::*;
use crate::simulation_world::chunk::WorldVoxelIteratorWithColumn;
use crate::simulation_world::{
    biome::BiomeRegistryResource,
    terrain::{
        biome::ClimateNoiseGenerator,
        components::{climate_map::TerrainClimateMapComponent, BiomeMapComponent},
        generators::core::BiomeGenerator,
    },
};

// A default implementation
#[derive(Debug, Default)]
pub struct DefaultBiomeGenerator;

impl BiomeGenerator for DefaultBiomeGenerator {
    #[instrument(skip_all)]
    fn generate_biome_chunk(
        &self,
        biome_map: &mut BiomeMapComponent,
        terrain_climate_map: &mut TerrainClimateMapComponent,
        iterator: WorldVoxelIteratorWithColumn,

        climate_noise: &ClimateNoiseGenerator,
        biome_registry: &BiomeRegistryResource,
    ) {
        let plains_id = biome_registry.get_biome_id_or_default("plains");
        let ocean_id = biome_registry.get_biome_id_or_default("ocean");

        let mut climate_data_for_column = Default::default();

        // --- Single loop using the smart iterator ---
        for item in iterator {
            // Destructure the local coordinates
            let (x, y, z) = item.local;

            // --- 1. 2D Column Work ---
            // Check if this is the first (y=0) voxel of a new (x, z) column
            if item.is_new_column {
                // Calculate 2D noise data *once* per column
                climate_data_for_column = climate_noise.get_climate_at(item.world.x, item.world.z);

                // Set the 2D climate map
                terrain_climate_map.set_data(x, z, climate_data_for_column.terrain_climate);
            }

            // --- 2. 3D Voxel Work ---
            // This code runs for every 'y' in the column,
            // reusing the cached 'climate_data_for_column'.
            // INFO: --------------------------
            //         Determine biomes
            // --------------------------------

            let temperature = climate_data_for_column.temperature;
            if temperature >= 0.5 {
                biome_map.set_data(x, y, z, plains_id);
            } else {
                biome_map.set_data(x, y, z, ocean_id);
            }
        }
    }
}
