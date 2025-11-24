use crate::prelude::*;
use crate::render_world::passes::world::gpu_resources::world_uniforms::ChunkStorageBindGroupLayout;
use crate::render_world::{
    graphics_context::resources::RenderDevice,
    passes::world::shadow_pass::gpu_resources::SHADOW_DEPTH_FORMAT,
};
use bevy_ecs::prelude::*;
use wesl::include_wesl;

use super::shadow_view_uniform::ShadowViewBindGroupLayout;

/// A resource that holds the shadow pass pipeline.
#[derive(Resource, Deref, DerefMut)]
pub struct ShadowPassPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl FromWorld for ShadowPassPipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let view_layout = world.resource::<ShadowViewBindGroupLayout>();
        let chunk_layout = world.resource::<ChunkStorageBindGroupLayout>();

        let shadow_pass_depth_stencil = Some(wgpu::DepthStencilState {
            format: SHADOW_DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            // small bias prevents some artifacts in exchange for
            // a small amount of peter panning which I am settling
            // for for now
            // bias: wgpu::DepthBiasState {
            //     constant: 1,
            //     slope_scale: 2.0,
            //     clamp: 1.0,
            // },
            bias: wgpu::DepthBiasState::default(),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shadow Pass Pipeline Layout"),
            bind_group_layouts: &[&view_layout.0, &chunk_layout.0],
            push_constant_ranges: &[],
        });

        let vs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shadow Pass Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("shadow_vert").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shadow Pass Pipeline"),
            layout: Some(&pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &vs_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                polygon_mode: wgpu::PolygonMode::Fill,
                // front cull prevents items from shadowing
                // themselves which is an awesome technique
                cull_mode: Some(wgpu::Face::Front),
                ..Default::default()
            },
            depth_stencil: shadow_pass_depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self { pipeline }
    }
}
