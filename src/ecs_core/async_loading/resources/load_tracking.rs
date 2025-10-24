use crate::prelude::*;
use bevy_ecs::resource::Resource;
use std::sync::{Arc, Mutex};

/// Loading tracker is a NonSend Resource that the outer app loop orchestrates
/// to enable both worlds to perform loading tasks in parallel before a state
/// transition occurs.
#[derive(Default)]
struct LoadingTrackerInner {
    pub is_simulation_ready: bool,
    pub is_renderer_ready: bool,
}

#[derive(Resource, Clone, Default)]
pub struct LoadingTracker {
    inner: Arc<Mutex<LoadingTrackerInner>>,
}

impl LoadingTracker {
    /// Returns true only if all worlds have reported that they are ready.
    pub fn is_all_ready(&self) -> bool {
        let guard = self.inner.lock().unwrap();
        guard.is_simulation_ready && guard.is_renderer_ready
    }

    pub fn is_simulation_ready(&self) -> bool {
        let guard = self.inner.lock().unwrap();
        guard.is_simulation_ready
    }

    pub fn is_renderer_ready(&self) -> bool {
        let guard = self.inner.lock().unwrap();
        guard.is_renderer_ready
    }

    /// Resets the tracker to its initial state.
    pub fn reset(&self) {
        let mut guard = self.inner.lock().unwrap();
        guard.is_simulation_ready = false;
        guard.is_renderer_ready = false;
    }

    // You'll need methods to set the flags now
    pub fn set_simulation_ready(&self, is_ready: bool) {
        self.inner.lock().unwrap().is_simulation_ready = is_ready;
    }

    pub fn set_renderer_ready(&self, is_ready: bool) {
        self.inner.lock().unwrap().is_renderer_ready = is_ready;
    }
}

#[derive(Resource, Default, Debug)]
pub struct LoadingTaskTracker {
    /// Number of tasks that have been spawned
    pub spawned: usize,
    /// Number of tasks that have completed
    pub completed: usize,
}

impl LoadingTaskTracker {
    /// Register that a task was spawned
    pub fn register_spawn(&mut self) {
        self.spawned += 1;
        debug!(
            target: "async_tasks",
            "[TRACKER] Task spawned: {}/{} completed",
            self.completed, self.spawned
        );
    }

    /// Register that a task completed
    pub fn register_completion(&mut self) {
        self.completed += 1;
        debug!(
            target: "async_tasks",
            "[TRACKER] Task completed: {}/{} done",
            self.completed, self.spawned
        );
    }

    /// Check if all spawned tasks have completed
    pub fn all_complete(&self) -> bool {
        self.spawned > 0 && self.completed >= self.spawned
    }

    /// Check if any tasks have been spawned yet
    pub fn has_spawned_tasks(&self) -> bool {
        self.spawned > 0
    }
}
