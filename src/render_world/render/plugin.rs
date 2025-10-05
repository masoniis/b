use super::render_scene_system;
use crate::{
    core::world::{EcsBuilder, Plugin},
    render_world::RenderSchedule,
};

pub struct PipelineModulePlugin;

impl Plugin for PipelineModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // schedules
        //     .entry(RenderSchedule::Render)
        //     .add_systems(render_loading_screen_system);
        builder
            .schedule_entry(RenderSchedule::Render)
            .add_systems(render_scene_system);
    }
}
