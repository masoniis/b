use bevy_ecs::prelude::Resource;
use std::collections::VecDeque;
use std::time::Duration;
use std::time::Instant;

#[derive(Resource)]
pub struct TimeResource {
    pub current: Instant,
    pub since_last_update: Duration,
    pub total_elapse: Duration,

    pub frame_duration_history: VecDeque<Duration>,
}

impl Default for TimeResource {
    fn default() -> Self {
        Self {
            current: Instant::now(),
            since_last_update: Duration::ZERO,
            total_elapse: Duration::ZERO,
            frame_duration_history: VecDeque::with_capacity(60),
        }
    }
}

impl TimeResource {
    pub fn average_frame_time(&self) -> Duration {
        if self.frame_duration_history.is_empty() {
            return Duration::ZERO;
        }
        let total: Duration = self.frame_duration_history.iter().sum();
        total / (self.frame_duration_history.len() as u32)
    }

    pub fn fps(&self) -> f32 {
        let avg_frame_time = self.average_frame_time();
        if avg_frame_time.as_secs_f32() > 0.0 {
            1.0 / avg_frame_time.as_secs_f32()
        } else {
            0.0
        }
    }
}
