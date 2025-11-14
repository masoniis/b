use crate::prelude::*;
use crate::render_world::graphics_context::resources::RenderDevice;
use crate::render_world::types::WireframeVertex;
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
pub fn setup_unit_bounding_box_mesh_system(mut commands: Commands, device: Res<RenderDevice>) {
    let dummy_color = [1.0, 1.0, 1.0];

    #[rustfmt::skip]
    let vertices: [WireframeVertex; 8] = [
        // Bottom face
        WireframeVertex::new([-0.5, -0.5, -0.5], dummy_color),
        WireframeVertex::new([0.5, -0.5, -0.5], dummy_color),
        WireframeVertex::new([0.5, -0.5, 0.5], dummy_color),
        WireframeVertex::new([-0.5, -0.5, 0.5], dummy_color),
        // Top face
        WireframeVertex::new([-0.5, 0.5, -0.5], dummy_color),
        WireframeVertex::new([0.5, 0.5, -0.5], dummy_color),
        WireframeVertex::new([0.5, 0.5, 0.5], dummy_color),
        WireframeVertex::new([-0.5, 0.5, 0.5],dummy_color),
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
