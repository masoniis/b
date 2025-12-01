use crate::simulation_world::terrain::{generators::shaping::lib::TerrainShaper, SinwaveShaper};
use bevy_ecs::prelude::Resource;
use std::sync::Arc;

/// A resource holding the active terrain chunk generator.
#[derive(Resource, Clone)]
pub struct ActiveTerrainGenerator(pub Arc<dyn TerrainShaper + Send + Sync>);

impl Default for ActiveTerrainGenerator {
    fn default() -> Self {
        ActiveTerrainGenerator(Arc::new(SinwaveShaper::new()))
    }
}
