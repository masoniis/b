use crate::render_world::{
    passes::ui_pass::startup::UiPipeline, resources::GraphicsContextResource,
};
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};
use std::num::NonZeroU64;

// This struct must match the memory layout of the uniforms in your shader.
// It needs to be aligned to 16 bytes for WGSL uniform rules. Mat4 and Vec4 are, so this is fine.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiInstanceData {
    pub model_matrix: [f32; 16], // 4x4 matrix
    pub color: [f32; 4],         // RGBA color
}

// This resource holds the single, large buffer for all UI panel instances.
#[derive(Resource)]
pub struct UiInstanceBuffer {
    /// The GPU buffer.
    pub buffer: wgpu::Buffer,
    /// The bind group that gives shaders access to the buffer.
    pub bind_group: wgpu::BindGroup,
    /// The stride between elements, considering alignment.
    pub stride: u32,
    /// A staging area on the CPU for this frame's data.
    pub instances: Vec<UiInstanceData>,
}

// A startup system to create the instance buffer resource.
pub fn setup_ui_instance_buffer(
    mut commands: Commands,
    gfx: Res<GraphicsContextResource>,
    pipeline: Res<UiPipeline>,
) {
    let device = &gfx.context.device;

    // Calculate the aligned size of our instance data. This is crucial for dynamic offsets.
    let stride = {
        let min_alignment = device.limits().min_uniform_buffer_offset_alignment;
        let instance_size = std::mem::size_of::<UiInstanceData>() as u32;
        (instance_size + min_alignment - 1) & !(min_alignment - 1)
    };

    // Pre-allocate a buffer that can hold a reasonable number of UI elements.
    let initial_capacity = 128;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("UI Instance Buffer"),
        size: (initial_capacity as u64) * (stride as u64),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("UI Instance Bind Group"),
        layout: &pipeline.node_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0, // instance data must be at binding 0 in @group(1)
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &buffer,
                offset: 0,
                // The size MUST be the aligned stride of a SINGLE element for dynamic offsets.
                size: NonZeroU64::new(stride as u64),
            }),
        }],
    });

    commands.insert_resource(UiInstanceBuffer {
        buffer,
        bind_group,
        stride,
        instances: Vec::with_capacity(initial_capacity),
    });
}
