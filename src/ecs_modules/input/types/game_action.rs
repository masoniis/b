/// Defines
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameAction {
    // Core player movement
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveFaster,

    // Misc
    ToggleDiagnostics,
}
