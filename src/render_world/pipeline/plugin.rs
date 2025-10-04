use super::systems::render_loading_screen_system;
use crate::{prelude::*, render_world::RenderSchedule};
use bevy_ecs::prelude::*;

pub struct PipelineModulePlugin;

impl Plugin for PipelineModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, _world: &mut World) {
        schedules
            .entry(RenderSchedule::Render)
            .add_systems(render_loading_screen_system);
    }
}
