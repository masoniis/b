use crate::render_world::graphics_context::resources::RenderDevice;
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};
use std::num::NonZeroU64;

// INFO: ----------------------
//         uniform data
// ----------------------------

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiMaterialData {
    pub color: [f32; 4],
}

// INFO: --------------------
//         gpu layout
// --------------------------

#[derive(Resource)]
pub struct UiMaterialBindGroupLayout(pub wgpu::BindGroupLayout);

impl FromWorld for UiMaterialBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("UI Material Bind Group Layout"),
            entries: &[
                // material uniform (color etc defined above)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: NonZeroU64::new(
                            std::mem::size_of::<UiMaterialData>() as u64
                        ),
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
pub struct UiMaterialBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub stride: u32,
    pub materials: Vec<UiMaterialData>,
}

impl FromWorld for UiMaterialBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let layout = world.resource::<UiMaterialBindGroupLayout>();

        let initial_capacity = 128;
        let stride = {
            let min_alignment = device.limits().min_uniform_buffer_offset_alignment;
            let instance_size = std::mem::size_of::<UiMaterialData>() as u32;
            (instance_size + min_alignment - 1) & !(min_alignment - 1)
        };

        let material_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("UI Material Buffer"),
            size: (initial_capacity as u64) * (stride as u64),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("UI Material Bind Group"),
            layout: &layout.0,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &material_buffer,
                    offset: 0,
                    size: NonZeroU64::new(stride as u64),
                }),
            }],
        });

        Self {
            buffer: material_buffer,
            bind_group: material_bind_group,
            stride,
            materials: Vec::with_capacity(initial_capacity),
        }
    }
}
