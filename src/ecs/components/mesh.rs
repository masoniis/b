use crate::graphics::GpuMesh;
use crate::graphics::webgpu_renderer::Vertex;
use bevy_ecs::prelude::Component;
use glam::Vec2;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Component)]
pub struct MeshComponent {
    pub gpu_mesh: Arc<GpuMesh>,

    pub atlas_id: String,
    pub uv_min: Vec2,
    pub uv_max: Vec2,
}

impl MeshComponent {
    /// Creates a new mesh from raw vertex and index data.
    pub fn new(gpu_mesh: &Arc<GpuMesh>, atlas_id: String, uv_min: Vec2, uv_max: Vec2) -> Self {
        Self {
            gpu_mesh: Arc::clone(gpu_mesh),
            atlas_id,
            uv_min,
            uv_max,
        }
    }
}

pub fn create_gpu_mesh_from_data(
    device: &wgpu::Device,
    vertices: &[Vertex],
    indices: &[u32],
) -> Arc<GpuMesh> {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    // Wrap the GpuMesh in an Arc
    Arc::new(GpuMesh {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
    })
}
