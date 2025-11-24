use crate::render_world::passes::world::gpu_resources::world_uniforms::{
    ChunkStorageManager, VoxelMesh,
};

use super::{PackedFace, WireframeVertex};
use wgpu::util::DeviceExt;

/// A type to connect ECS components to the webgpu renderer
pub struct GpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

pub fn create_gpu_mesh_from_data(
    device: &wgpu::Device,
    vertices: &[WireframeVertex],
    indices: &[u32],
) -> GpuMesh {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Mesh Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Mesh Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    GpuMesh {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
    }
}

/// Uploads a voxel mesh to the SSBO and returns its handle.
pub fn upload_voxel_mesh(
    manager: &mut ChunkStorageManager,
    queue: &wgpu::Queue,
    faces: &[PackedFace],
    world_pos: [f32; 3],
) -> Option<VoxelMesh> {
    manager.allocate_chunk(queue, faces, world_pos)
}
