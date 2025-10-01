// Manages the entire application's lifecycle
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Launching,
    Running,
    ShuttingDown,
}
