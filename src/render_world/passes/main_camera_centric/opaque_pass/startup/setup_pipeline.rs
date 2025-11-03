use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use crate::render_world::passes::core::create_render_pipeline::{
    CreatedPipeline, PipelineDefinition,
};
use crate::render_world::passes::core::create_render_pipeline_from_def;
use crate::render_world::passes::main_camera_centric::opaque_pass::startup::DEPTH_FORMAT;
use crate::render_world::passes::main_camera_centric::shared::{
    CentralCameraViewBindGroupLayout, EnvironmentBindGroupLayout,
};
use crate::render_world::types::vertex::Vertex;
use bevy_ecs::prelude::*;
use wesl::include_wesl;

/// A resource that holds all the opaque phase pipelines.
#[derive(Resource)]
pub struct OpaquePipelines {
    /// A pipeline that draws filled opaque geometry.
    pub fill: CreatedPipeline,
    /// A pipeline that draws wireframe opaque geometry.
    pub wireframe: CreatedPipeline,

    /// A pipeline that draws the skybox.
    pub skybox: CreatedPipeline,
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
    view_layout: Res<CentralCameraViewBindGroupLayout>, // common @group(0) layout
    environment_layout: Res<EnvironmentBindGroupLayout>,
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
    //        Regular opaque pipeline
    // ---------------------------------------

    let fill_pipeline_def = PipelineDefinition {
        label: "Opaque Pipeline",
        material_path: "assets/shaders/camera_centric/opaque/main.material.ron",
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

    let fill_pipeline: CreatedPipeline = create_render_pipeline_from_def(
        &device,
        &[&view_layout.0, &environment_layout.0],
        fill_pipeline_def,
    );

    // INFO: -----------------------------------
    //        Wireframe opaque pipeline
    // -----------------------------------------

    let wireframe_pipeline_def = PipelineDefinition {
        label: "Wireframe Opaque Pipeline",
        material_path: "assets/shaders/camera_centric/opaque/main.material.ron",
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

    let wireframe_pipeline: CreatedPipeline = create_render_pipeline_from_def(
        &device,
        &[&view_layout.0, &environment_layout.0],
        wireframe_pipeline_def,
    );

    // INFO: --------------------------------
    //        skybox opaque pipeline
    // --------------------------------------

    let skybox_depth_stencil = Some(wgpu::DepthStencilState {
        format: DEPTH_FORMAT,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::GreaterEqual,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    });

    let skybox_pipeline_def = PipelineDefinition {
        label: "Skybox Opaque Pipeline",
        material_path: "assets/shaders/camera_centric/skybox/main.material.ron",
        vs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("skybox_vert").into()),
        fs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("skybox_frag").into()),
        vertex_buffers: &[],
        fragment_targets: &opaque_fragment_target,
        depth_stencil: skybox_depth_stencil,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            cull_mode: None,
            ..Default::default()
        },
    };

    let skybox_pipeline: CreatedPipeline = create_render_pipeline_from_def(
        &device,
        &[&view_layout.0, &environment_layout.0],
        skybox_pipeline_def,
    );

    // INFO: -------------------------
    //        setup resources
    // -------------------------------

    commands.insert_resource(OpaquePipelines {
        fill: fill_pipeline,
        wireframe: wireframe_pipeline,
        skybox: skybox_pipeline,
    });

    commands.insert_resource(OpaqueRenderMode::default());
}
