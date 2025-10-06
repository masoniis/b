use crate::render_world::{
    passes::loading_pass::prepare::LoadingScreenPipelineLayoutsResource,
    resources::{GraphicsContextResource, PipelineCacheResource, PipelineId},
};
use bevy_ecs::prelude::*;
use std::fs;

// --- Constants ---
pub const SHADER_PATH: &str = "assets/shaders/scene/simple.wgsl";
pub const LOADING_SHADER_PATH: &str = "assets/shaders/loading_screen/loading.wgsl";
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub const LOADING_SCREEN_PIPELINE_ID: PipelineId = 1;

/// A one-time system that is completely self-contained for compiling pipelines.
/// It defines all the layouts it needs locally for MVP simplicity.
pub fn prepare_pipelines_system(
    mut cache: ResMut<PipelineCacheResource>,
    gfx_context: Res<GraphicsContextResource>,
    loading_layouts: Res<LoadingScreenPipelineLayoutsResource>,
) {
    let device = &gfx_context.context.device;
    let surface_format = gfx_context.context.config.format;

    // --- Create Loading Screen Pipeline ---
    if cache.get(LOADING_SCREEN_PIPELINE_ID).is_none() {
        // Compile Pipeline
        let shader_source =
            fs::read_to_string(LOADING_SHADER_PATH).expect("Failed to read loading shader file");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Loading Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Loading Screen Pipeline Layout"),
            bind_group_layouts: &[&loading_layouts.time_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Loading Screen Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            cache: None,
            multiview: None,
        });

        cache.insert(LOADING_SCREEN_PIPELINE_ID, pipeline);
    }
}
