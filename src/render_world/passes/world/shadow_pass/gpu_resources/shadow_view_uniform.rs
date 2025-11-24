use crate::render_world::graphics_context::resources::RenderDevice;
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

// INFO: ----------------------------
//         uniform definition
// ----------------------------------

/// The shadow "camera" (i.e., the sun's) view uniform.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct ShadowViewData {
    pub light_view_proj_matrix: [f32; 16],
}

// INFO: -----------------------------------------
//         GPU binding, buffer, and layout
// -----------------------------------------------

/// The shadow pass "sun camera" bind group layout.
#[derive(Resource)]
pub struct ShadowViewBindGroupLayout(pub wgpu::BindGroupLayout);

impl FromWorld for ShadowViewBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow View Bind Group Layout"),
            entries: &[
                // slot for `ShadowViewData` uniform defined above
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

/// A GPU buffer resource containing the shadow pass's view data.
#[derive(Resource)]
pub struct ShadowViewBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl FromWorld for ShadowViewBuffer {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let view_layout = world.resource::<ShadowViewBindGroupLayout>();

        let view_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shadow View Buffer"),
            size: std::mem::size_of::<ShadowViewData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shadow View Bind Group"),
            layout: &view_layout.0,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer: view_buffer,
            bind_group: view_bind_group,
        }
    }
}
