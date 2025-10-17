use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

use super::SimulationWorldPlaceholder;

/// A wrapper for the game world so it can be inserted as a resource in the render world.
#[derive(Resource)]
pub struct SimulationWorld {
    pub val: World,
}

/// A custom schedule runner for the `Extract` schedule.
///
/// This function works by temporarily swapping the `main_world` with an empty
/// "scratch world", and inserting the real `main_world` into the `render_world` as a
/// resource. Doesn't waste time on any allocations 😎
///
/// The `simulation_world` is returned to its original state after the schedule has run.
#[instrument(skip_all, fields(name = "run_extract_schedule"))]
pub fn run_extract_schedule(
    simulation_world: &mut World,
    render_world: &mut World,
    schedule_label: impl ScheduleLabel,
) {
    // At this point, GameWorldPlaceholder should be an empty GameWorld.
    let placeholder_world = simulation_world
        .remove_resource::<SimulationWorldPlaceholder>()
        .expect("ScratchMainWorld resource not found. Did you forget to call initialize_main_world_for_extract()?");

    // We swap the empty with the real one passed in, avoiding an allocation.
    let taken_main_world = std::mem::replace(simulation_world, placeholder_world.val);

    // Insert and run the schedule
    render_world.insert_resource(SimulationWorld {
        val: taken_main_world,
    });
    render_world.run_schedule(schedule_label);

    // Remove the world after running and swap back
    let main_world_resource = render_world
        .remove_resource::<SimulationWorld>()
        .expect("MainWorld resource was removed unexpectedly during extract schedule.");
    let new_scratch_world = std::mem::replace(simulation_world, main_world_resource.val);
    simulation_world.insert_resource(SimulationWorldPlaceholder {
        val: new_scratch_world,
    });
}
