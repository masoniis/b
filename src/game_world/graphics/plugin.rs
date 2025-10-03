use super::{
    resources::{CameraUniformResource, RenderQueueResource},
    systems::render,
    systems::render_queue,
};
use crate::{
    game_world::{Plugin, Schedules},
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

pub struct RenderingModulePlugin;

impl Plugin for RenderingModulePlugin {
    fn build(&self, schedules: &mut Schedules, world: &mut World) {
        world.insert_resource(RenderQueueResource::default());
        world.insert_resource(CameraUniformResource::default());

        schedules.main.add_systems((
            (
                render_queue::changed_mesh_system,
                render_queue::removed_mesh_system,
                render_queue::removed_screen_text_system,
            )
                .in_set(CoreSet::RenderPrep),
            (render::render_scene_system,).in_set(CoreSet::Render),
        ));
    }
}
