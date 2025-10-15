/// Loading tracker is a NonSend Resource that the outer app loop orchestrates
/// to enable both worlds to perform loading tasks in parallel before a state
/// transition occurs.
#[derive(Debug, Default)]
pub struct LoadingTracker {
    pub is_simulation_ready: bool,
    pub is_renderer_ready: bool,
}

impl LoadingTracker {
    /// Returns true only if all worlds have reported that they are ready.
    pub fn is_all_ready(&self) -> bool {
        self.is_simulation_ready && self.is_renderer_ready
    }

    /// Resets the tracker to its initial state.
    pub fn reset(&mut self) {
        self.is_simulation_ready = false;
        self.is_renderer_ready = false;
    }
}

/// Represents the loading status of a single world.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LoadStatus {
    /// The world is not currently loading and has no pending tasks.
    #[default]
    Idle,
    /// The world has active loading tasks that are not yet complete.
    Loading,
    /// The world has completed all its loading tasks (or had none to begin with).
    Ready,
}
