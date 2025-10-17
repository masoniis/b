use std::num::NonZero;

use crate::render_world::resources::{GraphicsContextResource, TextureArrayResource};
use crate::render_world::uniforms::{CameraUniform, ModelUniform};
use bevy_ecs::prelude::*;
use wgpu::util::DeviceExt;
use wgpu::BindGroup;

use super::MeshPipelineLayoutsResource;

pub const MAX_MODELS_PER_FRAME: u64 = 250;

#[derive(Resource)]
pub struct ModelBindGroup {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub stride: wgpu::DynamicOffset,
}

impl FromWorld for ModelBindGroup {
    fn from_world(world: &mut World) -> Self {
        let layouts = world
            .get_resource::<MeshPipelineLayoutsResource>()
            .expect("MeshPipelineLayouts not initialized");
        let gfx_context = world
            .get_resource::<GraphicsContextResource>()
            .expect("GraphicsContextResource not initialized");
        let device = &gfx_context.context.device;

        let min_alignment = device.limits().min_uniform_buffer_offset_alignment;
        let model_uniform_size = std::mem::size_of::<ModelUniform>() as u64;

        // Calculate the padded size of a single model uniform.
        // This is the correct stride value we must use.
        let stride = {
            let alignment = min_alignment as u64;
            (model_uniform_size + alignment - 1) & !(alignment - 1)
        };

        let buffer_size = MAX_MODELS_PER_FRAME * std::mem::size_of::<ModelUniform>() as u64;

        // Create an empty/default buffer for our initial "dummy" model uniform
        let model_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Model Uniform Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let model_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Model Bind Group"),
            layout: &layouts.model_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &model_buffer,
                    offset: 0,
                    size: NonZero::new(256), // Size of ONE element, not the whole buffer
                }),
            }],
        });

        Self {
            buffer: model_buffer,
            bind_group: model_bind_group,
            stride: stride.try_into().unwrap(),
        }
    }
}
/// A resource to hold the `BindGroup` for the camera's view/projection matrix.
/// This is updated every frame.
#[derive(Resource)]
pub struct ViewBindGroup {
    pub buffer: wgpu::Buffer,
    pub bind_group: BindGroup,
}

/// FromWorld
impl FromWorld for ViewBindGroup {
    fn from_world(world: &mut World) -> Self {
        let layouts = world.get_resource::<MeshPipelineLayoutsResource>().unwrap();
        let gfx_context = world.get_resource::<GraphicsContextResource>().unwrap();
        let device = &gfx_context.context.device;

        // Create an empty/default buffer for our initial "dummy" bind group
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Dummy Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[CameraUniform::new()]), // A zeroed-out matrix
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create the initial bind group
        let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Initial View Bind Group"),
            layout: &layouts.camera_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        Self {
            bind_group: view_bind_group,
            buffer: camera_buffer,
        }
    }
}

/// A resource to hold the `BindGroup` for the main texture atlas/array and its sampler.
/// This is typically created once at startup.
#[derive(Resource)]
pub struct MainTextureBindGroup(pub BindGroup);

impl FromWorld for MainTextureBindGroup {
    fn from_world(world: &mut World) -> Self {
        // --- Get Dependencies from the World ---
        let layouts = world
            .get_resource::<MeshPipelineLayoutsResource>()
            .expect("MeshPipelineLayouts not initialized");
        let gfx_context = world
            .get_resource::<GraphicsContextResource>()
            .expect("GraphicsContextResource not initialized");
        let texture_array = world
            .get_resource::<TextureArrayResource>()
            .expect("TextureArrayResource not initialized");

        let device = &gfx_context.context.device;

        // --- Create the BindGroup ---
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Main Texture Bind Group"),
            layout: &layouts.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_array.array.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_array.array.sampler),
                },
            ],
        });

        Self(texture_bind_group)
    }
}
