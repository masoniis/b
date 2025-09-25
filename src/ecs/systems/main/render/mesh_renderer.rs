use crate::ecs::components::{MeshComponent, TransformComponent};
use crate::ecs::resources::RenderQueueResource;
use crate::graphics::rendercore::QueuedDraw;
use bevy_ecs::prelude::{Query, ResMut};

pub fn mesh_render_system(
    mut render_queue: ResMut<RenderQueueResource>,
    query: Query<(&MeshComponent, &TransformComponent)>,
) {
    for (mesh_comp, transform_comp) in &query {
        let model_matrix = transform_comp.to_matrix();

        let queued_draw = QueuedDraw {
            mesh_handle: mesh_comp.mesh_handle,
            instance_count: 1,
            transform: model_matrix,
        };

        render_queue.add_scene_object(queued_draw);
    }
}
