use crate::render_world::graphics_context::resources::RenderDevice;
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

// INFO: ----------------------
//         uniform data
// ----------------------------

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiObjectData {
    pub model_matrix: [f32; 16],
}

// INFO: --------------------
//         gpu layout
// --------------------------

#[derive(Resource)]
pub struct UiObjectBindGroupLayout(pub wgpu::BindGroupLayout);

impl FromWorld for UiObjectBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("UI Object Bind Group Layout"),
            entries: &[
                // object uniform (model matrix defined above)
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

// INFO: --------------------
//         gpu buffer
// --------------------------

#[derive(Resource)]
pub struct UiObjectBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub objects: Vec<UiObjectData>,
}

impl FromWorld for UiObjectBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let layout = world.resource::<UiObjectBindGroupLayout>();

        let initial_capacity = 128;
        let object_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("UI Object Buffer"),
            size: (initial_capacity as u64) * std::mem::size_of::<UiObjectData>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("UI Object Bind Group"),
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
