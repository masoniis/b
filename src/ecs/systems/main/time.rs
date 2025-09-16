use crate::ecs::resources::TimeResource;
use bevy_ecs::prelude::*;
use std::time::Instant;

pub fn time_system(mut time: ResMut<TimeResource>) {
    let current_time = Instant::now();
    let delta = current_time.duration_since(time.last_update).as_secs_f32();

    time.delta_seconds = delta;
    time.last_update = current_time;
}
