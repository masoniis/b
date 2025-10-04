use crate::prelude::*;
use bevy_ecs::{
    prelude::*,
    schedule::ScheduleLabel,
    system::{SystemParam, SystemState},
};

/// A custom schedule runner for the `Extract` schedule.
///
/// This function runs each system in the `Extract` schedule on the `render_world`,
/// but it provides the system parameters (like `Res` and `Query`) from the `main_world`.
///
/// This allows extract systems to have clean, Bevy-like signatures.
pub fn run_extract_schedule(
    main_world: &World,
    render_world: &mut World,
    schedule_label: impl ScheduleLabel,
) {
    // Temporarily take the schedule out of the render world to run it.
    let mut schedule = match render_world.remove_schedule(&schedule_label) {
        Some(schedule) => schedule,
        None => {
            // If the schedule doesn't exist, there's nothing to do.
            return;
        }
    };

    // This is the key: we prepare the command queue for the render world.
    // Systems in the extract schedule will buffer their commands here.
    let mut render_commands = Commands::new(render_world);

    // Get the systems from the schedule. We need mutable access to run them.
    let systems = schedule.graph_mut().systems_mut();

    for system_id in systems.keys() {
        // Get the system by its ID. We have to do a bit of unsafe swapping to get a mutable reference.
        let system = systems.get_mut(system_id).unwrap();

        // Use SystemState to prepare the system's parameters from the MAIN world.
        // This is the "magic". SystemState fetches the data for Res, Query, etc.
        let mut system_state = SystemState::new(main_world);
        let params = system_state.get(main_world);

        // Run the system with the parameters from the main world, but provide
        // the command queue for the render world.
        system.run_with_commands(params, &mut render_commands);
    }

    // After all systems have run, apply their buffered commands to the render world.
    render_commands.apply(render_world);

    // Put the schedule back into the render world for the next frame.
    render_world.add_schedule(schedule);
}
