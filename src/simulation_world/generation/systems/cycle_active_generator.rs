use crate::simulation_world::generation::{
    ActiveTerrainGenerator, SinWaveGenerator, SuperflatGenerator, TerrainGenerator,
};
use bevy_ecs::system::{Local, ResMut};
use std::sync::Arc;

// A type alias for a function that constructs a terrain generator
type GeneratorConstructor = fn() -> Arc<dyn TerrainGenerator>;

// Simple constructor functions
fn new_superflat() -> Arc<dyn TerrainGenerator> {
    Arc::new(SuperflatGenerator::new())
}

fn new_sinwave() -> Arc<dyn TerrainGenerator> {
    Arc::new(SinWaveGenerator::new())
}

// This static array is our "map" of generators to cycle through
static GENERATOR_LIST: &[GeneratorConstructor] = &[
    new_superflat,
    new_sinwave,
    // Add new generator functions here, e.g., new_perlin_noise
];

/// A simple Bevy system that cycles through generators by creating a new one each time.
pub fn cycle_active_generator(
    mut active_generator: ResMut<ActiveTerrainGenerator>,
    mut current_index: Local<usize>,
) {
    if GENERATOR_LIST.is_empty() {
        return; // Do nothing if the list is empty
    }

    // Increment and wrap the index using system-local state
    *current_index = (*current_index + 1) % GENERATOR_LIST.len();

    // Get the constructor function from our list
    let constructor = GENERATOR_LIST[*current_index];

    // Call the constructor to create a brand new generator
    // and assign it to the active one.
    active_generator.0 = constructor();
}
