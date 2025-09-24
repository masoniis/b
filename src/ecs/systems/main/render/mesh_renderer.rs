use crate::ecs::components::{MeshComponent, TransformComponent};
use crate::graphics::webgpu_renderer::{QueuedDraw, WebGpuRenderer};
use bevy_ecs::prelude::{Query, ResMut};

pub fn mesh_render_system(
    mut renderer: ResMut<WebGpuRenderer>,
    query: Query<(&MeshComponent, &TransformComponent)>,
) {
    for (mesh_comp, transform_comp) in &query {
        let model_matrix = transform_comp.to_matrix();

        let queued_draw = QueuedDraw {
            gpu_mesh: mesh_comp.gpu_mesh.clone(),
            instance_count: 1,
            transform: model_matrix,
        };

        renderer.queue_draw(queued_draw);
    }
}
