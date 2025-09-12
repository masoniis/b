use crate::ecs::resources::TimeResource;
use shred::{System, Write};
use std::time::Instant;

pub struct TimeSystem;

impl<'a> System<'a> for TimeSystem {
    type SystemData = Write<'a, TimeResource>;

    fn run(&mut self, mut time: Self::SystemData) {
        let current_time = Instant::now();
        let delta = current_time.duration_since(time.last_update).as_secs_f32();

        time.delta_seconds = delta;
        time.last_update = current_time;
    }
}
