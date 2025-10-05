use crate::render_world::render::GraphicsContextResource;
use bevy_ecs::prelude::*;
use wgpu::BindGroupLayout;

/// A resource that holds the bind group layouts needed for the mesh pipeline.
#[derive(Resource)]
pub struct MeshPipelineLayoutsResource {
    pub camera_layout: BindGroupLayout,
    pub texture_layout: BindGroupLayout,
}

// This is the magic. `FromWorld` provides a one-time constructor for resources.
// It will be called automatically exactly once when we do `world.init_resource::<MeshPipeline>()`.
impl FromWorld for MeshPipelineLayoutsResource {
    fn from_world(world: &mut World) -> Self {
        // We can get other resources, like the device, from the world.
        let gfx_context = world.get_resource::<GraphicsContextResource>().unwrap();
        let device = &gfx_context.context.device;

        // Define the layouts here, in their one and only source location.
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

        MeshPipelineLayoutsResource {
            camera_layout,
            texture_layout,
        }
    }
}
