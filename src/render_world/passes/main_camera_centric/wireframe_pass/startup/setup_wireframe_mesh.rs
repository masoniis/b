use crate::prelude::*;
use crate::render_world::graphics_context::resources::RenderDevice;
use crate::render_world::types::Vertex;
use bevy_ecs::prelude::*;
use wgpu::util::DeviceExt;

#[derive(Resource)]
pub struct DebugWireframeMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

/// Creates a 1x1x1 wireframe cube mesh to be used for representing chunk bounding boxes.
#[instrument(skip_all)]
pub fn setup_wireframe_mesh_system(mut commands: Commands, device: Res<RenderDevice>) {
    let dummy_normal = [0.0, 1.0, 0.0];
    let dummy_uv = [0.0, 0.0];
    let dummy_tex_index = 0;

    let vertices: [Vertex; 8] = [
        // Bottom face
        Vertex::new([-0.5, -0.5, -0.5], dummy_normal, dummy_uv, dummy_tex_index), // 0
        Vertex::new([0.5, -0.5, -0.5], dummy_normal, dummy_uv, dummy_tex_index),  // 1
        Vertex::new([0.5, -0.5, 0.5], dummy_normal, dummy_uv, dummy_tex_index),   // 2
        Vertex::new([-0.5, -0.5, 0.5], dummy_normal, dummy_uv, dummy_tex_index),  // 3
        // Top face
        Vertex::new([-0.5, 0.5, -0.5], dummy_normal, dummy_uv, dummy_tex_index), // 4
        Vertex::new([0.5, 0.5, -0.5], dummy_normal, dummy_uv, dummy_tex_index),  // 5
        Vertex::new([0.5, 0.5, 0.5], dummy_normal, dummy_uv, dummy_tex_index),   // 6
        Vertex::new([-0.5, 0.5, 0.5], dummy_normal, dummy_uv, dummy_tex_index),  // 7
    ];

    // buffer
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Debug Wireframe Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    // 12 lines, 2 indices per line
    #[rustfmt::skip]
    let indices: [u32; 24] = [
        0, 1, 1, 2, 2, 3, 3, 0,
        4, 5, 5, 6, 6, 7, 7, 4,
        0, 4, 1, 5, 2, 6, 3, 7,
    ];

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Debug Wireframe Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    commands.insert_resource(DebugWireframeMesh {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
    });
}
