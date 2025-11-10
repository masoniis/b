pub mod biome_map;
pub mod climate_map;
pub mod height_maps;

pub use biome_map::BiomeMapComponent;
pub use climate_map::TerrainClimateMapComponent;
pub use height_maps::{OceanFloorHeightMapComponent, WorldSurfaceHeightMapComponent};
