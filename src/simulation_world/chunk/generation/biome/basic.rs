use crate::simulation_world::chunk::core::{BiomeGenerator, GeneratedBiomeData};
use crate::simulation_world::chunk::{
    BiomeMap, ClimateMap,
};
use glam::IVec3;
use std::fmt::Debug;

// A default implementation
#[derive(Debug, Default)]
pub struct DefaultBiomeGenerator;

impl BiomeGenerator for DefaultBiomeGenerator {
    fn generate_biome_data(&self, _coord: IVec3) -> GeneratedBiomeData {
        return GeneratedBiomeData {
            biome_map: BiomeMap::empty(),
            climate_map: ClimateMap::empty(),
        };
    }
}
