use crate::{prelude::*, render_world::RenderSchedule};
use bevy_ecs::prelude::*;

pub struct QueueModulePlugin;

impl Plugin for QueueModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, _world: &mut World) {
        schedules.entry(RenderSchedule::Queue);
    }
}
