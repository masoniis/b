/// Defines
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SimulationAction {
    // Core player movement
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveFaster,

    // Core player interaction
    BreakVoxel,
    PlaceVoxel,

    // Terrain interactions
    CycleActiveTerrainGenerator,

    // Time control interactions
    JumpGameTimeForward,
    JumpGameTimeBackward,
    PauseGameTime,

    // Misc
    ToggleDiagnostics,
    ToggleOpaqueWireframeMode,
    ToggleChunkBorders,
    TogglePause,
}
