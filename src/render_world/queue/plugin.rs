use crate::{
    core::world::{EcsBuilder, Plugin},
    render_world::RenderSchedule,
};

use super::queue_mesh_system;
use crate::render_world::queue::resources::queue::RenderQueueResource;

pub struct QueueModulePlugin;

impl Plugin for QueueModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(RenderQueueResource::default());

        builder
            .schedule_entry(RenderSchedule::Queue)
            .add_systems(queue_mesh_system);
    }
}
