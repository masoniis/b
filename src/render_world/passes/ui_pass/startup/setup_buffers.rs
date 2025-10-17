use crate::prelude::*;
use crate::render_world::{
    passes::ui_pass::startup::UiPipeline, resources::GraphicsContextResource,
};
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};
use std::num::NonZeroU64;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiMaterialData {
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UiObjectData {
    pub model_matrix: [f32; 16],
}

#[derive(Resource)]
pub struct UiMaterialBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub stride: u32,
    pub materials: Vec<UiMaterialData>,
}

#[derive(Resource)]
pub struct UiObjectBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub objects: Vec<UiObjectData>,
}

#[instrument(skip_all)]
pub fn setup_ui_buffers(
    mut commands: Commands,
    gfx: Res<GraphicsContextResource>,
    pipeline: Res<UiPipeline>,
) {
    let device = &gfx.context.device;
    let initial_capacity = 128;

    // Create material buffer
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
        layout: &pipeline.material_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &material_buffer,
                offset: 0,
                size: NonZeroU64::new(stride as u64),
            }),
        }],
    });

    commands.insert_resource(UiMaterialBuffer {
        buffer: material_buffer,
        bind_group: material_bind_group,
        stride,
        materials: Vec::with_capacity(initial_capacity),
    });

    // Create object buffer
    let object_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("UI Object Buffer"),
        size: (initial_capacity as u64) * std::mem::size_of::<UiObjectData>() as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("UI Object Bind Group"),
        layout: &pipeline.object_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: object_buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(UiObjectBuffer {
        buffer: object_buffer,
        bind_group: object_bind_group,
        objects: Vec::with_capacity(initial_capacity),
    });
}
