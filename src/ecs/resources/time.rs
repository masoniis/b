use std::time::Instant;

pub struct TimeResource {
    pub delta_seconds: f32,
    pub last_update: Instant,
}

impl Default for TimeResource {
    fn default() -> Self {
        Self {
            delta_seconds: 0.0,
            last_update: Instant::now(),
        }
    }
}
