use crate::prelude::*;
use crate::render_world::passes::core::create_render_pipeline::create_render_pipeline_from_def;
use crate::render_world::passes::core::view::ViewBindGroupLayout;
use crate::render_world::passes::core::CreatedPipeline;
use crate::render_world::types::vertex::Vertex;
use crate::render_world::{
    graphics_context::resources::RenderDevice, textures::resource::TextureArrayResource,
};
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};
use wesl::include_wesl;

// INFO: -------------------------------------------------
//         Resources for the Transparent Rendering Pass
// -------------------------------------------------------

#[derive(Resource)]
pub struct TransparentPipeline {
    pub pipeline: CreatedPipeline,
}

#[derive(Resource)]
pub struct TransparentMaterialBindGroup(pub wgpu::BindGroup);

#[derive(Resource)]
pub struct TransparentObjectBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub objects: Vec<TransparentObjectData>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct TransparentObjectData {
    pub model_matrix: [f32; 16],
}

// INFO: -------------------------------------------------
//         System for Initializing the Transparent Pass
// -------------------------------------------------------

#[instrument(skip_all)]
pub fn startup_transparent_pass_system(
    mut commands: Commands,
    device: Res<RenderDevice>,
    texture_array_resource: Res<TextureArrayResource>,
    view_bind_group_layout: Res<ViewBindGroupLayout>,
) {
    let created_pipeline = create_render_pipeline_from_def(
        &device,
        &view_bind_group_layout,
        crate::render_world::passes::core::create_render_pipeline::PipelineDefinition {
            label: "Transparent Render Pipeline",
            material_path: "assets/shaders/transparent/main.material.ron",
            vs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("transparent_vert").into()),
            fs_shader_source: wgpu::ShaderSource::Wgsl(include_wesl!("transparent_frag").into()),
            vertex_buffers: &[Vertex::desc()],
            fragment_targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::GreaterEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
        },
    );

    // bind group

    let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Transparent Material Bind Group"),
        layout: &created_pipeline.material_layout(),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_array_resource.array.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture_array_resource.array.sampler),
            },
        ],
    });
    commands.insert_resource(TransparentMaterialBindGroup(material_bind_group));

    // buffer

    let initial_capacity = 100; // Start with a reasonable capacity
    let initial_size = (initial_capacity * std::mem::size_of::<TransparentObjectData>()) as u64;

    let object_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Transparent Object Buffer"),
        size: initial_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Transparent Object Bind Group"),
        layout: &created_pipeline.object_layout(),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: object_buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(TransparentObjectBuffer {
        buffer: object_buffer,
        bind_group: object_bind_group,
        objects: Vec::with_capacity(initial_capacity),
    });

    commands.insert_resource(TransparentPipeline {
        pipeline: created_pipeline,
    });
}
