use bevy_ecs::prelude::Resource;
use std::time::{Duration, Instant};

use crate::render_world::extract::utils::ExtractResource;

// Lower values are smoother, higher values are more responsive.
const FPS_SMOOTHING_FACTOR: f32 = 0.025;

#[derive(Resource)]
pub struct TimeResource {
    pub current: Instant,
    pub since_last_update: Duration,
    pub total_elapse: Duration,
    pub smoothed_fps: f32,
}

impl Default for TimeResource {
    fn default() -> Self {
        Self {
            current: Instant::now(),
            since_last_update: Duration::ZERO,
            total_elapse: Duration::ZERO,
            smoothed_fps: 69.0,
        }
    }
}

#[derive(Resource)]
pub struct RenderTime {
    pub delta_seconds: f32,
}

impl ExtractResource for RenderTime {
    type Source = TimeResource;
    type Output = Self; // The output is a RenderTime resource

    fn extract_resource(source: &Self::Source) -> Self::Output {
        RenderTime {
            delta_seconds: source.total_elapse.as_secs_f32(),
        }
    }
}

impl TimeResource {
    pub fn update_fps(&mut self) {
        let current_raw_fps = if self.since_last_update.as_secs_f32() > 0.0 {
            1.0 / self.since_last_update.as_secs_f32()
        } else {
            0.0
        };

        // Uses EMA to update FPS
        self.smoothed_fps = (current_raw_fps * FPS_SMOOTHING_FACTOR)
            + (self.smoothed_fps * (1.0 - FPS_SMOOTHING_FACTOR));
    }
}
