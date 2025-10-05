use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

/// A wrapper for the game world so it can be inserted as a resource in the render world.
#[derive(Resource)]
pub struct GameWorld {
    pub val: World,
}

/// A "scratch" world used to avoid allocating new worlds every frame when
/// swapping out the `MainWorld` for the `Extract` schedule.
#[derive(Resource, Default)]
struct GameWorldPlaceholder {
    val: World,
}

/// Initializes the main world with the necessary resources for the extract runner.
/// This must be called before running anything in the mainworld if we want to render.
pub fn initialize_main_world_for_extract(main_world: &mut World) {
    main_world.init_resource::<GameWorldPlaceholder>();
}

/// A custom schedule runner for the `Extract` schedule.
///
/// This function works by temporarily swapping the `main_world` with an empty
/// "scratch world", and inserting the real `main_world` into the `render_world` as a
/// resource. Doesn't waste time on any allocations 😎
///
/// The `game_world` is returned to its original state after the schedule has run.
pub fn run_extract_schedule(
    game_world: &mut World,
    render_world: &mut World,
    schedule_label: impl ScheduleLabel,
) {
    // At this point, GameWorldPlaceholder should be an empty GameWorld.
    let placeholder_world = game_world
        .remove_resource::<GameWorldPlaceholder>()
        .expect("ScratchMainWorld resource not found. Did you forget to call initialize_main_world_for_extract()?");

    // We swap the empty with the real one passed in, avoiding an allocation.
    let taken_main_world = std::mem::replace(game_world, placeholder_world.val);

    // Insert and run the schedule
    render_world.insert_resource(GameWorld {
        val: taken_main_world,
    });
    render_world.run_schedule(schedule_label);

    // Remove the world after running and swap back
    let main_world_resource = render_world
        .remove_resource::<GameWorld>()
        .expect("MainWorld resource was removed unexpectedly during extract schedule.");
    let new_scratch_world = std::mem::replace(game_world, main_world_resource.val);
    game_world.insert_resource(GameWorldPlaceholder {
        val: new_scratch_world,
    });
}
