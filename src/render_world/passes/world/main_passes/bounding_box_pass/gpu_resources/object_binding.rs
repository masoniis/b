use crate::render_world::graphics_context::resources::RenderDevice;
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

// INFO: ----------------------------
//         uniform definition
// ----------------------------------

/// The per-object data (model matrix) for a single wireframe instance.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WireframeObjectData {
    pub model_matrix: [f32; 16],
}

// INFO: -----------------------------------------
//         GPU binding, buffer, and layout
// -----------------------------------------------

#[derive(Resource)]
pub struct WireframeObjectBindGroupLayout(pub wgpu::BindGroupLayout);

impl FromWorld for WireframeObjectBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Wireframe Object Layout"),
            entries: &[
                // object storage buffer (matrices as seen above)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        Self(layout)
    }
}

/// A resource holding the GPU buffer and bind group for wireframe object data.
#[derive(Resource)]
pub struct WireframeObjectBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub objects: Vec<WireframeObjectData>,
}

impl FromWorld for WireframeObjectBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let layout = world.resource::<WireframeObjectBindGroupLayout>();

        let initial_capacity = 128;
        let object_buffer_size =
            (initial_capacity as u64) * std::mem::size_of::<WireframeObjectData>() as u64;

        let object_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Wireframe Object Buffer"),
            size: object_buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Wireframe Object Bind Group"),
            layout: &layout.0,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: object_buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer: object_buffer,
            bind_group: object_bind_group,
            objects: Vec::with_capacity(initial_capacity),
        }
    }
}
