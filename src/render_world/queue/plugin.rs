use crate::{prelude::*, render_world::RenderSchedule};
use bevy_ecs::prelude::*;

use super::queue_mesh_system;
use crate::render_world::queue::resources::queue::RenderQueueResource;

pub struct QueueModulePlugin;

impl Plugin for QueueModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, world: &mut World) {
        world.insert_resource(RenderQueueResource::default());

        schedules
            .entry(RenderSchedule::Queue)
            .add_systems(queue_mesh_system);
    }
}
