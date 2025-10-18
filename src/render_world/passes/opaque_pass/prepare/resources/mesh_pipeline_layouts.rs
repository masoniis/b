use crate::render_world::graphics_context::resources::RenderDevice;
use bevy_ecs::prelude::*;
use wgpu::BindGroupLayout;

/// A resource that holds the bind group layouts needed for the mesh pipeline.
#[derive(Resource)]
pub struct MeshPipelineLayoutsResource {
    pub camera_layout: BindGroupLayout,
    pub texture_layout: BindGroupLayout,
    pub model_layout: BindGroupLayout,
}

impl FromWorld for MeshPipelineLayoutsResource {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().expect(
            "
            The RenderDevice is required to create the MeshPipelineLayoutsResource.
            ",
        );

        let device = &device.0;

        // INFO: Camera bind
        let camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // INFO: Texture array bind
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // INFO: Model matrix bind
        let model_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Model Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        MeshPipelineLayoutsResource {
            camera_layout,
            model_layout,
            texture_layout,
        }
    }
}
