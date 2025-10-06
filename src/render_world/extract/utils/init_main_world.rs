use bevy_ecs::prelude::*;

/// A "scratch" world used to avoid allocating new worlds every frame when
/// swapping out the `MainWorld` for the `Extract` schedule.
#[derive(Resource, Default)]
pub struct GameWorldPlaceholder {
    pub val: World,
}

/// Initializes the main world with the necessary resources for the extract runner.
/// This must be called before running anything in the mainworld if we want to render.
pub fn initialize_main_world_for_extract(main_world: &mut World) {
    main_world.init_resource::<GameWorldPlaceholder>();
}
