use crate::render_world::graphics_context::resources::RenderDevice;
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};
use std::num::NonZeroU64;

// INFO: ----------------------
//         uniform data
// ----------------------------

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiViewData {
    pub projection_matrix: [f32; 16],
}

// INFO: --------------------
//         gpu layout
// --------------------------

#[derive(Resource)]
pub struct UiViewBindGroupLayout(pub wgpu::BindGroupLayout);

impl FromWorld for UiViewBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("UI View Bind Group Layout"),
            entries: &[
                // view uniform (projection matrix)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
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
pub struct UiViewBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl FromWorld for UiViewBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let layout = world.resource::<UiViewBindGroupLayout>();

        let view_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("UI View Buffer"),
            size: std::mem::size_of::<UiViewData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("UI View Bind Group"),
            layout: &layout.0,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &view_buffer,
                    offset: 0,
                    size: NonZeroU64::new(std::mem::size_of::<UiViewData>() as u64),
                }),
            }],
        });

        Self {
            buffer: view_buffer,
            bind_group: view_bind_group,
        }
    }
}
