use crate::prelude::*;
use crate::render_world::graphics_context::resources::RenderDevice;
use bevy_ecs::prelude::*;
use wgpu::util::DeviceExt;

/// A resource for a wgpu mesh that fills the entire screen.
///
/// This quad represents a unit quad that every UI element can use
/// by applying their respective model matrix to scale/position it.
#[derive(Resource)]
pub struct ScreenQuadResource {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl FromWorld for ScreenQuadResource {
    #[instrument(skip_all)]
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let device = &device.0;

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

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }
}
