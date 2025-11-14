use crate::prelude::*;
use crate::render_world::{
    graphics_context::resources::RenderDevice,
    passes::core::{create_render_pipeline_from_def, CreatedPipeline, PipelineDefinition},
    passes::world::shadow_pass::startup::SHADOW_DEPTH_FORMAT,
    types::vertex::WorldVertex,
};
use bevy_ecs::prelude::*;
use wesl::include_wesl;

/// A resource that holds the shadow pass pipeline.
#[derive(Resource, Deref, DerefMut)]
pub struct ShadowPassPipeline {
    pub pipeline: CreatedPipeline,
}

/// Setup the Opaque pipeline using the shader and material definition
#[instrument(skip_all)]
pub fn setup_shadow_pass_pipeline(
    // Input
    device: Res<RenderDevice>,

    // Output (spawned resource)
    mut commands: Commands,
) {
    let shadow_pass_depth_stencil = Some(wgpu::DepthStencilState {
        format: SHADOW_DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilState::default(),
        // add small bias during shadow pass
        bias: wgpu::DepthBiasState {
            constant: 2,
            slope_scale: 2.0,
            clamp: 0.0,
        },
    });

    // INFO: ------------------------------
    //         shadow pass pipeline
    // ------------------------------------

    let shadow_pass_pipeline_def = PipelineDefinition {
        label: "Shadow Pass Pipeline",
        material_path: "assets/shaders/world/shadow/main.material.ron",
        vs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("shadow_vert").into()),
        fs_shader_source: None,
        vertex_buffers: &[WorldVertex::desc()],
        fragment_targets: &[],
        depth_stencil: shadow_pass_depth_stencil,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
    };

    let pipeline: CreatedPipeline =
        create_render_pipeline_from_def(&device, &[], shadow_pass_pipeline_def);

    // INFO: -------------------------
    //         setup resources
    // -------------------------------

    commands.insert_resource(ShadowPassPipeline { pipeline });
}
