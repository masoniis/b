use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use crate::render_world::passes::core::create_render_pipeline::{
    CreatedPipeline, PipelineDefinition,
};
use crate::render_world::passes::core::{create_render_pipeline_from_def, ViewBindGroupLayout};
use crate::render_world::passes::opaque_pass::startup::DEPTH_FORMAT;
use crate::render_world::types::vertex::Vertex;
use bevy_ecs::prelude::*;
use wesl::include_wesl;

/// A resource that holds all the opaque pipelines. Currently, this includes the
/// default "fill rasterization" and alternative "wireframe rasterization" pipelines.
#[derive(Resource)]
pub struct OpaquePipelines {
    pub fill: CreatedPipeline,
    pub wireframe: CreatedPipeline,
}

/// A resource that defines the current opaque render mode
#[derive(Resource, Default, Debug, PartialEq)]
pub enum OpaqueRenderMode {
    #[default]
    Fill,
    Wireframe,
}

/// Setup the Opaque pipeline using the shader and material definition
#[instrument(skip_all)]
pub fn setup_opaque_pipelines(
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

    // INFO: ---------------------------------
    //         Regular opaque pipeline
    // ---------------------------------------

    let fill_pipeline_def = PipelineDefinition {
        label: "Opaque Pipeline",
        material_path: "assets/shaders/opaque/main.material.ron",
        vs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("opaque_vert").into()),
        fs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("opaque_frag").into()),
        vertex_buffers: &[Vertex::desc()],
        fragment_targets: &opaque_fragment_target,
        depth_stencil: opaque_depth_stencil.clone(),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
    };

    let fill_pipeline: CreatedPipeline =
        create_render_pipeline_from_def(&device, &view_layout, fill_pipeline_def);

    // INFO: -----------------------------------
    //         Wireframe opaque pipeline
    // -----------------------------------------

    let wireframe_pipeline_def = PipelineDefinition {
        label: "Wireframe Opaque Pipeline",
        material_path: "assets/shaders/opaque/main.material.ron",
        vs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("opaque_vert").into()),
        fs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("opaque_frag").into()),
        vertex_buffers: &[Vertex::desc()],
        fragment_targets: &opaque_fragment_target,
        depth_stencil: opaque_depth_stencil,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Line,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
    };

    let wireframe_pipeline: CreatedPipeline =
        create_render_pipeline_from_def(&device, &view_layout, wireframe_pipeline_def);

    // INFO: -------------------------
    //         setup resources
    // -------------------------------

    commands.insert_resource(OpaquePipelines {
        fill: fill_pipeline,
        wireframe: wireframe_pipeline,
    });

    commands.insert_resource(OpaqueRenderMode::default());
}
