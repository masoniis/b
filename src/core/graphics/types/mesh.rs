use crate::core::graphics::types::vertex::Vertex;

use wgpu::util::DeviceExt;

/// A type to connect ECS components to the webgpu renderer
pub struct GpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

pub fn create_gpu_mesh_from_data(
    device: &wgpu::Device,
    vertices: &[Vertex],
    indices: &[u32],
) -> GpuMesh {
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
    GpuMesh {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
    }
}
