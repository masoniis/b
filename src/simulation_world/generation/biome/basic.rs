use glam::IVec3;
use std::fmt::Debug;

use crate::simulation_world::generation::{
    core::{BiomeGenerator, GeneratedBiomeData},
    BiomeMapComponent, ClimateMapComponent,
};

// A default implementation
#[derive(Debug, Default)]
pub struct DefaultBiomeGenerator;

impl BiomeGenerator for DefaultBiomeGenerator {
    fn generate_biome_data(&self, _coord: IVec3) -> GeneratedBiomeData {
        return GeneratedBiomeData {
            biome_map: BiomeMapComponent::empty(),
            climate_map: ClimateMapComponent::empty(),
        };
    }
}
