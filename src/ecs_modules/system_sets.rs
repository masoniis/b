use bevy_ecs::prelude::SystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSet {
    // For systems that create initial assets and entities.
    InitialSetup,
    // For the final system that transitions the AppState.
    Finalize,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CoreSet {
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
