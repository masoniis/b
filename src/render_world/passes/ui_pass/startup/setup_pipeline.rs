use super::ViewBindGroupLayout;
use crate::render_world::material::MaterialDefinition;
use crate::render_world::resources::GraphicsContextResource;
use bevy_ecs::prelude::*;
use naga;
use std::collections::BTreeMap;
use std::fs;

/// A resource to hold the pipeline and bind group layouts for our UI shader.
#[derive(Resource)]
pub struct UiPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub view_bind_group_layout: wgpu::BindGroupLayout,
    pub node_bind_group_layout: wgpu::BindGroupLayout,
}

const SHADER_PATH: &str = "assets/shaders/ui/main.wgsl";
const MATERIAL_PATH: &str = "assets/shaders/ui/main.material.ron";

// Setup the UI pipeline using the shader and material definition
pub fn setup_ui_pipeline(
    mut commands: Commands,
    gfx: Res<GraphicsContextResource>,
    view_layout: Res<ViewBindGroupLayout>,
) {
    let device = &gfx.context.device;
    let surface_format = gfx.context.config.format;

    // Process shader files (including parse)
    let shader_source = fs::read_to_string(SHADER_PATH).expect("Failed to read UI shader file");
    let material_source = fs::read_to_string(MATERIAL_PATH).expect("Failed to read material file");
    let material_def: MaterialDefinition =
        ron::from_str(&material_source).expect("Failed to parse material definition");
    let _shader_module =
        naga::front::wgsl::parse_str(&shader_source).expect("Failed to parse shader");

    // TODO: validate the shader against the material_def here.

    // Generate layouts from the metadata
    let mut created_layouts: BTreeMap<u32, wgpu::BindGroupLayout> = BTreeMap::new();

    for (&group_index, layout_def) in &material_def.bind_group_layouts {
        let entries: Vec<wgpu::BindGroupLayoutEntry> = layout_def
            .bindings
            .iter()
            .map(|binding_def| create_layout_entry_from_metadata(binding_def))
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&format!("UI Bind Group Layout @group({})", group_index)),
            entries: &entries,
        });
        created_layouts.insert(group_index, layout);
    }

    // --- 4. Assemble Final Pipeline Layout using Conventions ---
    let mut pipeline_bind_group_layouts = BTreeMap::new();
    // For @group(0), use the "well-known" engine layout
    pipeline_bind_group_layouts.insert(0, &view_layout.0);
    // For other groups, use the layouts we just generated from the RON file
    for (group_index, layout) in &created_layouts {
        pipeline_bind_group_layouts.insert(*group_index, layout);
    }
    let pipeline_bind_group_layouts_ref: Vec<&wgpu::BindGroupLayout> =
        pipeline_bind_group_layouts.values().copied().collect();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("UI Pipeline Layout"),
        bind_group_layouts: &pipeline_bind_group_layouts_ref,
        push_constant_ranges: &[],
    });

    // Create the RenderPipeline (as before)
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("UI Shader"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
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
                format: surface_format,
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

    // --- 6. Store the final resources ---
    commands.insert_resource(UiPipeline {
        pipeline,
        view_bind_group_layout: view_layout.0.clone(),
        node_bind_group_layout: created_layouts
            .remove(&1)
            .expect("Material missing @group(1)"),
    });
}

// Helper function to convert our RON definition into a wgpu layout entry
fn create_layout_entry_from_metadata(
    binding_def: &crate::render_world::material::BindingDef,
) -> wgpu::BindGroupLayoutEntry {
    let visibility = binding_def
        .visibility
        .iter()
        .fold(wgpu::ShaderStages::NONE, |acc, stage| {
            acc | match stage.as_str() {
                "Vertex" => wgpu::ShaderStages::VERTEX,
                "Fragment" => wgpu::ShaderStages::FRAGMENT,
                "Compute" => wgpu::ShaderStages::COMPUTE,
                _ => panic!("Unknown shader visibility stage"),
            }
        });

    let ty = match binding_def.ty.as_str() {
        "Buffer" => {
            let opts = binding_def
                .buffer_options
                .as_ref()
                .expect("Buffer must have buffer_options");
            wgpu::BindingType::Buffer {
                ty: match opts.ty.as_str() {
                    "Uniform" => wgpu::BufferBindingType::Uniform,
                    "Storage" => wgpu::BufferBindingType::Storage { read_only: false },
                    _ => panic!("Unknown buffer type"),
                },
                has_dynamic_offset: opts.has_dynamic_offset,
                min_binding_size: None,
            }
        }
        // TODO:  add cases for "Texture" and "Sampler"
        _ => panic!("Unsupported binding type"),
    };

    wgpu::BindGroupLayoutEntry {
        binding: binding_def.binding,
        visibility,
        ty,
        count: None,
    }
}
