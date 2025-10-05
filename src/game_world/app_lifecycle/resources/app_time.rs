use bevy_ecs::prelude::*;
use std::time::{Duration, Instant};

/// A resource that tracks the running time of the app.
///
/// This should track time from the moment the app is launched
/// to the moment it is closed without interruption. The state
/// (being paused or something) should not impact the time tracking.
#[derive(Resource)]
pub struct AppTimeResource {
    pub last_update: Instant,
    pub delta: Duration,
    pub elapsed_unscaled: Duration,
}

impl Default for AppTimeResource {
    fn default() -> Self {
        AppTimeResource {
            last_update: Instant::now(),
            delta: Duration::ZERO,
            elapsed_unscaled: Duration::ZERO,
        }
    }
}
