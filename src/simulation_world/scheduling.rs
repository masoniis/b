use crate::ecs_core::state_machine::State;
use bevy_ecs::prelude::SystemSet;
use bevy_ecs::schedule::ScheduleLabel;

// INFO: -------------------------
//         Core scheduling
// -------------------------------

/// Core pre-defined schedule labels
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationSchedule {
    /// A schedule that runs once on application startup.
    Startup,
    /// A schedule that runs continuously the game is loading resources, worlds, etc.
    Loading,
    /// A schedule that runs at a fixed timestep based on tickrate, ideal for physics and game logic.
    FixedUpdate,
    /// A schedule that runs every frame variably, ideal for non-simulation state logic.
    Main,
}

/// The sets for the fixed schedule of the
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FixedUpdateSet {
    /// Handle state transitions and other pre-logic tasks.
    PreUpdate,
    /// The main game logic: player movement, AI, block breaking, etc.
    MainLogic,
}

/// The core schedule sets for the simulation world.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SimulationSet {
    /// Process raw OS input and publish game-specific events.
    Input,
    /// Handle state transitions and other pre-logic tasks.
    PreUpdate,
    /// The main game logic: player movement, AI, block breaking, etc.
    Update,
    /// Physics, collision detection, and character controller logic.
    Physics,
    /// Cleanup after physics and logic (e.g., syncing transforms).
    PostUpdate,
    /// Collect all data needed for rendering into queues/buffers.
    RenderPrep,
}

// INFO: --------------------------------------
//         Generic transition schedules
// --------------------------------------------

/// Schedule that runs once in the entering state.
///
/// It will run before any other system in that state.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnEnter<T: State>(pub T);

/// Schedule that runs once in the exiting state as we transition to a new state.
///
/// This means it will run before any system in the new state we are entering.
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OnExit<T: State>(pub T);
