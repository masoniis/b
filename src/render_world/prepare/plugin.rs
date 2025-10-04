use crate::{
    prelude::*, render_world::extract::resources::RenderMeshStorageResource,
    render_world::RenderSchedule,
};
use bevy_ecs::prelude::*;

use super::systems::prepare_meshes_system;

pub struct PrepareModulePlugin;

impl Plugin for PrepareModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, world: &mut World) {
        world.insert_resource(RenderMeshStorageResource::default());

        schedules
            .entry(RenderSchedule::Prepare)
            .add_systems(prepare_meshes_system);
    }
}
