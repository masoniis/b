use crate::prelude::*;
use crate::simulation_world::terrain::shaping::{RealisticShaper, SimplexShaper};
use crate::simulation_world::terrain::{
    ActiveTerrainGenerator, NoisyShaper, SinwaveShaper, SuperflatShaper, TerrainShaper,
};
use bevy_ecs::{
    resource::Resource,
    system::{Local, Res, ResMut},
};
use std::sync::Arc;

#[derive(Resource, Default)]
pub struct TerrainGeneratorLibrary {
    pub generators: Vec<Arc<dyn TerrainShaper + Send + Sync>>,
}

/// A system that sets up the terrain generator by loading a default set of generators
/// into it.
pub fn setup_terrain_gen_library(mut lib: ResMut<TerrainGeneratorLibrary>) {
    lib.generators.push(Arc::new(SinwaveShaper::new()));
    lib.generators.push(Arc::new(SuperflatShaper::new()));
    lib.generators.push(Arc::new(NoisyShaper::new()));
    lib.generators.push(Arc::new(SimplexShaper::new()));
    lib.generators.push(Arc::new(RealisticShaper::new()));
}

/// A simple startup system that sets the default terrain generator to avoid confusion
/// regarding the default state of the `ActiveTerrainGenerator` resource.
pub fn set_default_terrain_generator(
    mut active_generator: ResMut<ActiveTerrainGenerator>,
    library: Res<TerrainGeneratorLibrary>,
) {
    active_generator.0 = library.generators[0].clone();
}

/// A simple system that cycles through terran generators (shapers).
pub fn cycle_active_generator(
    mut active_generator: ResMut<ActiveTerrainGenerator>,
    library: Res<TerrainGeneratorLibrary>,
    mut current_index: Local<usize>,
) {
    *current_index = (*current_index + 1) % library.generators.len();
    active_generator.0 = library.generators[*current_index].clone();

    info!("Switched to generator index: {}", *current_index);
}
