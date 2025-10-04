use crate::{
    core::graphics::types::QueuedDraw,
    game_world::graphics::{MeshComponent, RenderQueueResource, TransformComponent},
    prelude::*,
};
use bevy_ecs::prelude::{Changed, Entity, Or, Query, ResMut};

/// handles both adding new meshes and updating existing ones that have changed.
pub fn changed_mesh_system(
    mut render_queue: ResMut<RenderQueueResource>,
    query: Query<
        (Entity, &MeshComponent, &TransformComponent),
        Or<(Changed<MeshComponent>, Changed<TransformComponent>)>,
    >,
) {
    for (entity, mesh_comp, transform_comp) in query.iter() {
        if let Some(queued_draw) = render_queue.get_scene_object_mut(&entity) {
            debug!(target: "mesh_sync", "Updating mesh for entity {:?}", entity);
            queued_draw.mesh_handle = mesh_comp.mesh_handle;
            queued_draw.transform = transform_comp.to_matrix();
        } else {
            debug!(target: "mesh_sync", "Adding new mesh for entity {:?}", entity);

            let queued_draw = QueuedDraw {
                entity,
                mesh_handle: mesh_comp.mesh_handle,
                instance_count: 1,
                transform: transform_comp.to_matrix(),
            };
            render_queue.add_scene_object(entity, queued_draw);
        }
    }
}
