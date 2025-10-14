use bevy_ecs::prelude::*;

/// A "scratch" world used to avoid allocating new worlds every frame when
/// swapping out the `MainWorld` for the `Extract` schedule.
#[derive(Resource, Default)]
pub struct SimulationWorldPlaceholder {
    pub val: World,
}

/// Initializes the main world with the necessary resources for the extract runner.
/// This must be called before running anything in the mainworld if we want to render.
pub fn initialize_simulation_world_for_extract(simulation_world: &mut World) {
    simulation_world.init_resource::<SimulationWorldPlaceholder>();
}
