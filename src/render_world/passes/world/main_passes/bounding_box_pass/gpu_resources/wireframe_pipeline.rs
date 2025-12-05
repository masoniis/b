use super::object_binding::WireframeObjectBindGroupLayout;
use crate::{
    prelude::*,
    render_world::{
        graphics_context::resources::{RenderDevice, RenderSurfaceConfig},
        passes::world::main_passes::shared_resources::{
            main_depth_texture::MAIN_DEPTH_FORMAT, CentralCameraViewBindGroupLayout,
            EnvironmentBindGroupLayout,
        },
        types::WireframeVertex,
    },
};
use bevy_ecs::prelude::*;
use wesl::include_wesl;

/// A resource holding the pipeline for rendering debug wireframes.
#[derive(Resource, Deref, DerefMut)]
pub struct WireframePipeline {
    pub inner: wgpu::RenderPipeline,
}

impl FromWorld for WireframePipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let config = world.resource::<RenderSurfaceConfig>();

        let view_layout = world.resource::<CentralCameraViewBindGroupLayout>();
        let environment_layout = world.resource::<EnvironmentBindGroupLayout>();
        let object_layout = world.resource::<WireframeObjectBindGroupLayout>();

        let wireframe_fragment_target = [Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let wireframe_depth_stencil = Some(wgpu::DepthStencilState {
            format: MAIN_DEPTH_FORMAT,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::GreaterEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Wireframe Pipeline Layout"),
            bind_group_layouts: &[&view_layout.0, &environment_layout.0, &object_layout.0],
            push_constant_ranges: &[],
        });

        let vs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Wireframe Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("wireframe_vert").into()),
        });

        let fs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Wireframe Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("wireframe_frag").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Wireframe Pipeline"),
            layout: Some(&pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &vs_shader,
                entry_point: Some("vs_main"),
                buffers: &[WireframeVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_shader,
                entry_point: Some("fs_main"),
                targets: &wireframe_fragment_target,
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: wireframe_depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self { inner: pipeline }
    }
}
