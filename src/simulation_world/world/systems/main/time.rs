use crate::simulation_world::global_resources::time::TimeResource;
use bevy_ecs::prelude::*;
use std::time::Instant;
use tracing::debug;

pub fn time_system(mut time: ResMut<TimeResource>) {
    let current_time = Instant::now();
    let since_last_update = current_time.duration_since(time.current);

    time.current = current_time;
    time.since_last_update = since_last_update;
    time.total_elapse += since_last_update;
    time.update_fps();

    debug!(target : "fps", "FPS: {:?}", time.smoothed_fps);
}
