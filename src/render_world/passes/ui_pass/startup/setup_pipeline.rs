use crate::prelude::*;
use crate::render_world::passes::core::create_render_pipeline::{
    CreatedPipeline, PipelineDefinition,
};
use crate::render_world::passes::core::create_render_pipeline_from_def;
use crate::render_world::passes::ui_pass::startup::ViewBindGroupLayout;
use crate::render_world::resources::GraphicsContextResource;
use bevy_ecs::prelude::*;
use wesl::include_wesl;

/// A resource to hold the pipeline and bind group layouts for our UI shader.
#[derive(Resource)]
pub struct UiPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub view_bind_group_layout: wgpu::BindGroupLayout,
    pub material_bind_group_layout: wgpu::BindGroupLayout,
    pub object_bind_group_layout: wgpu::BindGroupLayout,
}

const UI_VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: (2 * std::mem::size_of::<f32>()) as wgpu::BufferAddress, // only 2d points for ui
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
};

/// Setup the UI pipeline using the shader and material definition
#[instrument(skip_all)]
pub fn setup_ui_pipeline(
    mut commands: Commands,
    gfx: Res<GraphicsContextResource>,
    view_layout: Res<ViewBindGroupLayout>,
) {
    let device = &gfx.context.device;

    // define the specific fragment target for UI (with alpha blending)
    let ui_fragment_target = [Some(wgpu::ColorTargetState {
        format: gfx.context.config.format,
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    // create the pipeline
    let ui_pipeline_def = PipelineDefinition {
        label: "UI Pipeline",
        material_path: "assets/shaders/ui/main.material.ron",
        vs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("ui_main_vert").into()),
        fs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("ui_main_frag").into()),
        vertex_buffers: &[UI_VERTEX_BUFFER_LAYOUT],
        fragment_targets: &ui_fragment_target,
        depth_stencil: None,
    };
    let created: CreatedPipeline =
        create_render_pipeline_from_def(device, &view_layout.0, ui_pipeline_def);

    // Insert the specific resource
    commands.insert_resource(UiPipeline {
        pipeline: created.pipeline,
        view_bind_group_layout: created.view_bind_group_layout,
        material_bind_group_layout: created.material_bind_group_layout,
        object_bind_group_layout: created.object_bind_group_layout,
    });
}
