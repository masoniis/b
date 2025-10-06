use bevy_ecs::prelude::*;
use wgpu::BindGroupLayout;

use crate::render_world::resources::GraphicsContextResource;

/// A resource that holds the bind group layouts needed for the mesh pipeline.
#[derive(Resource)]
pub struct LoadingScreenPipelineLayoutsResource {
    pub time_layout: BindGroupLayout,
}

impl FromWorld for LoadingScreenPipelineLayoutsResource {
    fn from_world(world: &mut World) -> Self {
        let gfx_context = world.get_resource::<GraphicsContextResource>().expect(
            "
            The GraphicsContextResource is required to create the MeshPipelineLayoutsResource.
            ",
        );

        let device = &gfx_context.context.device;

        let time_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Time Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        LoadingScreenPipelineLayoutsResource {
            time_layout: time_bind_group_layout,
        }
    }
}
