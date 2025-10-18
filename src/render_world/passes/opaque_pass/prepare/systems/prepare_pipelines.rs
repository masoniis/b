use crate::render_world::{
    graphics_context::resources::{RenderDevice, RenderSurfaceConfig},
    passes::opaque_pass::prepare::MeshPipelineLayoutsResource,
    resources::{PipelineCacheResource, PipelineId},
    types::vertex::Vertex,
};
use bevy_ecs::prelude::*;
use tracing::instrument;
use wesl::include_wesl;

// --- Constants ---
pub const LOADING_SHADER_PATH: &str = "assets/shaders/loading_screen/loading.wesl";
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub const MESH_PIPELINE_ID: PipelineId = 0;

/// A one-time system that is completely self-contained for compiling pipelines.
/// It defines all the layouts it needs locally for MVP simplicity.
#[instrument(skip_all)]
pub fn prepare_pipelines_system(
    mut cache: ResMut<PipelineCacheResource>,
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,
    mesh_layouts: Res<MeshPipelineLayoutsResource>,
) {
    let device = &device.0;
    let surface_format = config.0.format;

    // --- Create Main Mesh Render Pipeline ---
    if cache.get(MESH_PIPELINE_ID).is_none() {
        // Compile Pipeline
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Mesh Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("scene_main").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Mesh Render Pipeline Layout"),
                bind_group_layouts: &[
                    &mesh_layouts.camera_layout,
                    &mesh_layouts.texture_layout,
                    &mesh_layouts.model_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Mesh Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
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
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::GreaterEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            cache: None,
            multiview: None,
        });

        cache.insert(MESH_PIPELINE_ID, render_pipeline);
    }
}
