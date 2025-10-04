use crate::{
    core::graphics::types::gpu_queues::QueuedDraw,
    render_world::{
        extract::extract_meshes::{RenderMeshComponent, RenderTransformComponent},
        queue::resources::queue::RenderQueueResource,
    },
};
use bevy_ecs::prelude::*;

/// The system responsible for populating the `RenderQueueResource`.
///
/// It runs during the `Queue` schedule, after the `Extract` schedule has finished.
/// It queries for all entities that have been extracted into the render world
/// and adds them to a list of draw calls for the renderer to consume.
pub fn queue_mesh_system(
    mut render_queue: ResMut<RenderQueueResource>,
    // This query runs on the render world's entities.
    meshes_query: Query<(Entity, &RenderMeshComponent, &RenderTransformComponent)>,
) {
    // It's often a good idea to clear the queue at the start of the system
    // to ensure no data from the previous frame persists.
    render_queue.clear_object_queue();

    for (entity, mesh, transform) in meshes_query.iter() {
        // info!("Queueing mesh for entity: {:?}", entity);
        // info!(" - Mesh Handle: {:?}", mesh.mesh_handle);
        // info!(" - Transform: {:?}", transform.transform);
        let queued_draw = QueuedDraw {
            entity,
            mesh_handle: mesh.mesh_handle,
            instance_count: 1, // Assuming 1 instance per entity for now
            transform: transform.transform,
        };
        render_queue.add_scene_object(entity, queued_draw);
    }
}
