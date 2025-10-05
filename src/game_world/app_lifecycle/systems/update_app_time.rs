use std::time::Instant;

use crate::game_world::app_lifecycle::AppTimeResource;
use bevy_ecs::prelude::*;

/// This system must be run exactly ONCE per application loop.
pub fn update_app_time_system(mut time: ResMut<AppTimeResource>) {
    let now = Instant::now();
    let delta = now.duration_since(time.last_update);

    time.delta = delta;
    time.elapsed_unscaled += delta;
    time.last_update = now;
}
