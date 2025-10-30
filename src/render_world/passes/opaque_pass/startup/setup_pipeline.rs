use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use crate::render_world::passes::core::create_render_pipeline::{
    CreatedPipeline, PipelineDefinition,
};
use crate::render_world::passes::core::{create_render_pipeline_from_def, ViewBindGroupLayout};
use crate::render_world::passes::opaque_pass::startup::DEPTH_FORMAT;
use crate::render_world::types::vertex::Vertex;
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use wesl::include_wesl;

#[derive(Resource, Deref, DerefMut)]
pub struct OpaquePipeline {
    pub inner: CreatedPipeline,
}

/// Setup the Opaque pipeline using the shader and material definition
#[instrument(skip_all)]
pub fn setup_opaque_pipeline(
    mut commands: Commands,
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,
    view_layout: Res<ViewBindGroupLayout>, // common @group(0) layout
) {
    let opaque_fragment_target = [Some(wgpu::ColorTargetState {
        format: config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    let opaque_depth_stencil = Some(wgpu::DepthStencilState {
        format: DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::GreaterEqual,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    });

    let opaque_pipeline_def = PipelineDefinition {
        label: "Opaque Pipeline",
        material_path: "assets/shaders/opaque/main.material.ron",
        vs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("opaque_main_vert").into()),
        fs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("opaque_main_frag").into()),
        vertex_buffers: &[Vertex::desc()],
        fragment_targets: &opaque_fragment_target,
        depth_stencil: opaque_depth_stencil,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
    };

    let created_pipeline: CreatedPipeline =
        create_render_pipeline_from_def(&device, &view_layout, opaque_pipeline_def);

    commands.insert_resource(OpaquePipeline {
        inner: created_pipeline,
    });
}
