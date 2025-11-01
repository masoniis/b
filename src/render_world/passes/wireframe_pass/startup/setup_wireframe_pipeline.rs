use crate::{
    prelude::*,
    render_world::{
        graphics_context::resources::{RenderDevice, RenderSurfaceConfig},
        passes::core::{
            create_render_pipeline_from_def, CreatedPipeline, PipelineDefinition,
            ViewBindGroupLayout,
        },
        passes::opaque_pass::startup::DEPTH_FORMAT,
        types::Vertex,
    },
};
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};
use wesl::include_wesl;

/// A resource holding the pipeline for rendering debug wireframes.
#[derive(Resource, Deref, DerefMut)]
pub struct WireframePipeline {
    pub inner: CreatedPipeline,
}

/// A resource holding the GPU buffer and bind group for wireframe object data.
#[derive(Resource)]
pub struct WireframeObjectBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub objects: Vec<WireframeObjectData>,
}

/// The per-object data (model matrix) for a single wireframe instance.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WireframeObjectData {
    pub model_matrix: [f32; 16],
}

/// A startup system that creates the `WireframePipeline` and `WireframeObjectBuffer`.
#[instrument(skip_all)]
pub fn setup_wireframe_pipeline_and_buffers(
    mut commands: Commands,
    device: Res<RenderDevice>,
    config: Res<RenderSurfaceConfig>,
    view_layout: Res<ViewBindGroupLayout>, // shared @group(0) camera layout
) {
    let wireframe_fragment_target = [Some(wgpu::ColorTargetState {
        format: config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    let wireframe_depth_stencil = Some(wgpu::DepthStencilState {
        format: DEPTH_FORMAT,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::GreaterEqual,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    });

    // INFO: ----------------------------
    //         spawn the pipeline
    // ----------------------------------

    let wireframe_pipeline_def = PipelineDefinition {
        label: "Wireframe Pipeline",
        material_path: "assets/shaders/wireframe/main.material.ron",
        vs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("wireframe_vert").into()),
        fs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("wireframe_frag").into()),
        vertex_buffers: &[Vertex::desc()],
        fragment_targets: &wireframe_fragment_target,
        depth_stencil: wireframe_depth_stencil,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            ..Default::default()
        },
    };

    let created_pipeline =
        create_render_pipeline_from_def(&device, &view_layout, wireframe_pipeline_def);

    // INFO: ------------------------------
    //         create object buffer
    // ------------------------------------

    let initial_capacity = 128;
    let object_buffer_size =
        (initial_capacity as u64) * std::mem::size_of::<WireframeObjectData>() as u64;

    let object_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Wireframe Object Buffer"),
        size: object_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Wireframe Object Bind Group"),
        layout: created_pipeline.get_layout(1),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: object_buffer.as_entire_binding(),
        }],
    });

    // INFO: --------------------------
    //         insert resources
    // --------------------------------

    commands.insert_resource(WireframePipeline {
        inner: created_pipeline,
    });

    commands.insert_resource(WireframeObjectBuffer {
        buffer: object_buffer,
        bind_group: object_bind_group,
        objects: Vec::with_capacity(initial_capacity),
    });
}
