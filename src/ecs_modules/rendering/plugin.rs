use super::{
    resources::{CameraUniformResource, RenderQueueResource},
    systems::main,
};
use crate::{
    ecs_modules::{Plugin, Schedules},
    prelude::CoreSet,
};
use bevy_ecs::prelude::*;

pub struct RenderingModulePlugin;

impl Plugin for RenderingModulePlugin {
    fn build(&self, schedules: &mut Schedules, world: &mut World) {
        world.insert_resource(RenderQueueResource::default());
        world.insert_resource(CameraUniformResource::default());

        schedules.main.add_systems(
            (
                main::changed_mesh_system,
                main::removed_mesh_system,
                main::removed_screen_text_system,
            )
                .in_set(CoreSet::RenderPrep),
        );
        schedules
            .main
            .add_systems(main::render::render_system.in_set(CoreSet::RenderPrep));
    }
}
