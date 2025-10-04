use super::{extract_camera_system, extract_resource_system, RenderTimeResource};
use crate::{prelude::*, render_world::RenderSchedule};
use bevy_ecs::prelude::*;

pub struct ExtractModulePlugin;

impl Plugin for ExtractModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, world: &mut World) {
        world.insert_resource(RenderTimeResource::default());

        schedules.entry(RenderSchedule::Extract).add_systems((
            extract_camera_system,
            extract_resource_system::<RenderTimeResource>,
        ));
    }
}
