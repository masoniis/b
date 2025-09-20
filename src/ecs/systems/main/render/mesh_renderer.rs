use crate::ecs::components::{MeshComponent, TransformComponent};
use crate::graphics::webgpu_renderer::{QueuedDraw, Vertex, WebGpuRenderer};
use bevy_ecs::prelude::{Query, ResMut};
use glam::Vec3;

pub fn mesh_render_system(
    mut renderer: ResMut<WebGpuRenderer>,
    query: Query<(&MeshComponent, &TransformComponent)>,
) {
    for (mesh_comp, transform_comp) in &query {
        let model_matrix = transform_comp.to_matrix();
        let mut transformed_webgpu_vertices: Vec<Vertex> = Vec::with_capacity(mesh_comp.webgpu_vertices.len());

        for vertex in &mesh_comp.webgpu_vertices {
            let position = Vec3::from(vertex.position);
            let transformed_position = model_matrix.transform_point3(position);

            transformed_webgpu_vertices.push(Vertex {
                position: transformed_position.to_array(),
                color: vertex.color, // Keep the color from the MeshComponent
            });
        }

        let queued_draw = QueuedDraw {
            vertices: transformed_webgpu_vertices,
            indices: Some(mesh_comp.webgpu_indices.clone()),
            instance_count: 1,
        };

        renderer.queue_draw(queued_draw);
    }
}
