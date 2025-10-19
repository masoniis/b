use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use crate::render_world::passes::core::create_render_pipeline::{
    CreatedPipeline, PipelineDefinition,
};
use crate::render_world::passes::core::create_render_pipeline_from_def;
use crate::render_world::passes::core::setup_view_layout::ViewBindGroupLayout;
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use wesl::include_wesl;

/// A resource to hold the pipeline and bind group layouts for our UI shader.
#[derive(Resource, Deref, DerefMut)]
pub struct UiPipeline {
    inner: CreatedPipeline,
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
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,
    view_layout: Res<ViewBindGroupLayout>,
) {
    let device = &device;

    // define the specific fragment target for UI (with alpha blending)
    let ui_fragment_target = [Some(wgpu::ColorTargetState {
        format: config.format,
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
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
    };

    let created_pipeline: CreatedPipeline =
        create_render_pipeline_from_def(device, &view_layout, ui_pipeline_def);

    commands.insert_resource(UiPipeline {
        inner: created_pipeline,
    });
}
