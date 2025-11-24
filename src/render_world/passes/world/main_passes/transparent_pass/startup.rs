use super::super::shared_resources::{
    CentralCameraViewBindGroupLayout, EnvironmentBindGroupLayout,
};
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use crate::render_world::passes::world::gpu_resources::world_uniforms::ChunkStorageBindGroupLayout;
use crate::render_world::passes::world::main_passes::shared_resources::main_depth_texture::MAIN_DEPTH_FORMAT;
use crate::render_world::passes::world::main_passes::shared_resources::TextureArrayBindGroupLayout;
use bevy_ecs::prelude::*;
use wesl::include_wesl;

// INFO: -------------------
//         resources
// -------------------------

#[derive(Resource)]
pub struct TransparentPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl FromWorld for TransparentPipeline {
    fn from_world(world: &mut World) -> Self {
        // deps
        let device = world.resource::<RenderDevice>();
        let config = world.resource::<RenderSurfaceConfig>();

        // layouts
        let view_layout = world.resource::<CentralCameraViewBindGroupLayout>();
        let environment_layout = world.resource::<EnvironmentBindGroupLayout>();
        let texture_layout = world.resource::<TextureArrayBindGroupLayout>();
        let chunk_storage_layout = world.resource::<ChunkStorageBindGroupLayout>();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Transparent Pipeline Layout"),
            bind_group_layouts: &[
                &view_layout.0,
                &environment_layout.0,
                &texture_layout.0,
                &chunk_storage_layout.0,
            ],
            push_constant_ranges: &[],
        });

        let vs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Transparent Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("transparent_vert").into()),
        });

        let fs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Transparent Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("transparent_frag").into()),
        });

        // define pipelene
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Transparent Render Pipeline"),
            layout: Some(&pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &vs_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.0.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: MAIN_DEPTH_FORMAT,
                depth_write_enabled: false, // transparent objects don't write depth
                depth_compare: wgpu::CompareFunction::GreaterEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self { pipeline }
    }
}
