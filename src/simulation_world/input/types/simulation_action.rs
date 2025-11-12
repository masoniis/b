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

    // Terrain interactions
    CycleActiveTerrainGenerator,

    // Misc
    ToggleDiagnostics,
    ToggleOpaqueWireframeMode,
    ToggleChunkBorders,
    TogglePause,
}
