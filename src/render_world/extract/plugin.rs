use super::extract_camera_system;
use crate::{prelude::*, render_world::RenderSchedule};
use bevy_ecs::prelude::*;

pub struct ExtractModulePlugin;

impl Plugin for ExtractModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, _world: &mut World) {
        schedules
            .entry(RenderSchedule::Extract)
            .add_systems(extract_camera_system);
    }
}
