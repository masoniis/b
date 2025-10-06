use crate::{
    game_world::ui::components::UiMaterial,
    prelude::*,
    render_world::{
        extract::{extract_component::ExtractedItems, ui::UiNodeExtractor},
        passes::ui_pass::queue::{RenderPhase, UiPhaseItem},
        resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;
use std::fs;
use wgpu::util::DeviceExt;

/// A resource to hold the pipeline and bind group layouts for our UI shader.
#[derive(Resource)]
pub struct UiPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub view_bind_group_layout: wgpu::BindGroupLayout,
    pub node_bind_group_layout: wgpu::BindGroupLayout,
}

/// A resource holding the shared projection matrix for the UI.
#[derive(Resource)]
pub struct UiViewBindGroup {
    pub bind_group: wgpu::BindGroup,
}

/// The fully prepared GPU data for a single UI node.
pub struct PreparedUiNode {
    pub bind_group: wgpu::BindGroup,
}

/// A resource to store all prepared UI nodes for this frame.
#[derive(Resource, Default)]
pub struct PreparedUiNodes {
    pub nodes: Vec<PreparedUiNode>,
}

// This system prepares the uniform buffers and bind groups for each UI node.
pub fn prepare_ui_nodes_system(
    mut commands: Commands,
    gfx: Res<GraphicsContextResource>,
    extracted_nodes: Res<ExtractedItems<UiNodeExtractor>>,
    pipeline: Res<UiPipeline>,
) {
    let device = &gfx.context.device;

    let mut prepared_nodes = Vec::new();

    for extracted_node in &extracted_nodes.items {
        if let UiMaterial::SolidColor { color } = extracted_node.material {
            let position = extracted_node.layout.position;
            let size = extracted_node.layout.size;
            let model_matrix =
                Mat4::from_translation(position.extend(0.0)) * Mat4::from_scale(size.extend(1.0));

            let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("UI Model Uniform Buffer"),
                contents: bytemuck::cast_slice(model_matrix.as_ref()),
                usage: wgpu::BufferUsages::UNIFORM,
            });

            let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("UI Color Uniform Buffer"),
                contents: bytemuck::cast_slice(&color),
                usage: wgpu::BufferUsages::UNIFORM,
            });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("UI Node Bind Group"),
                layout: &pipeline.node_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: model_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: color_buffer.as_entire_binding(),
                    },
                ],
            });

            prepared_nodes.push(PreparedUiNode { bind_group });
        }
    }

    commands.insert_resource(PreparedUiNodes {
        nodes: prepared_nodes,
    });
}

const SHADER_PATH: &str = "assets/shaders/ui/main.wgsl";
pub fn setup_ui_pipeline(mut commands: Commands, gfx: Res<GraphicsContextResource>) {
    let device = &gfx.context.device;
    let surface_format = &gfx.context.config.format;

    let shader_source = fs::read_to_string(SHADER_PATH).expect("Failed to read mesh shader file");
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("UI Shader"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });

    let view_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("UI View Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                // Projection Matrix
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let node_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("UI Node Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    // Model Matrix
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    // Color
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("UI Pipeline Layout"),
        bind_group_layouts: &[&view_bind_group_layout, &node_bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("UI Pipeline"),
        layout: Some(&pipeline_layout),
        cache: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: (2 * std::mem::size_of::<f32>()) as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x2],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: *surface_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    commands.insert_resource(UiPipeline {
        pipeline,
        view_bind_group_layout,
        node_bind_group_layout,
    });

    commands.init_resource::<RenderPhase<UiPhaseItem>>();
}
