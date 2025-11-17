use crate::simulation_world::terrain::{
    ActiveTerrainGenerator, SinWaveGenerator, SuperflatShaper, TerrainShaper,
};
use bevy_ecs::system::{Local, ResMut};
use std::sync::Arc;

/// A type alias for a function that constructs a terrain generator
type GeneratorConstructor = fn() -> Arc<dyn TerrainShaper>;

// generator constructors
fn new_superflat() -> Arc<dyn TerrainShaper> {
    Arc::new(SuperflatShaper::new())
}
fn new_sinwave() -> Arc<dyn TerrainShaper> {
    Arc::new(SinWaveGenerator::new())
}

static GENERATOR_LIST: &[GeneratorConstructor] = &[new_superflat, new_sinwave];

/// A simple system that cycles through generators by creating a new one each time.
pub fn cycle_active_generator(
    mut active_generator: ResMut<ActiveTerrainGenerator>,
    mut current_index: Local<usize>,
) {
    *current_index = (*current_index + 1) % GENERATOR_LIST.len();
    let constructor = GENERATOR_LIST[*current_index];
    active_generator.0 = constructor();
}
