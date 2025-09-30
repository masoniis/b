/// Defines
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameAction {
    ToggleDiagnostics,
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}
