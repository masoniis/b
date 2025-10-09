use crate::render_world::resources::GraphicsContextResource;
use bevy_ecs::prelude::*;
use wgpu::util::DeviceExt;

/// A resource for a wgpu mesh that fills the entire screen.
#[derive(Resource)]
pub struct ScreenQuadResource {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

/// A one-time system that creates the shared quad mesh on the GPU.
///
/// The quad is intended to be the root canvas for all drawing, and
/// as such it covers the entire screen.
pub fn setup_ui_screen_quad_system(
    // Input
    gfx: Res<GraphicsContextResource>,

    // Output (spawn entity)
    mut commands: Commands,
) {
    let device = &gfx.context.device;

    let vertices: &[f32] = &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
    let indices: &[u16] = &[0, 3, 2, 0, 2, 1];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("UI Quad Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("UI Quad Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    commands.insert_resource(ScreenQuadResource {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
    });
}
