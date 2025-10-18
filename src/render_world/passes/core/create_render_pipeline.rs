use crate::render_world::types::{BindingDef, MaterialDefinition};
use std::collections::BTreeMap;
use std::fs;

pub struct PipelineDefinition<'a> {
    pub label: &'a str,
    pub material_path: &'a str,
    pub vs_shader_source: wgpu::ShaderSource<'a>,
    pub fs_shader_source: wgpu::ShaderSource<'a>,
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub fragment_targets: &'a [Option<wgpu::ColorTargetState>],
    pub depth_stencil: Option<wgpu::DepthStencilState>,
}

pub struct CreatedPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub view_bind_group_layout: wgpu::BindGroupLayout,
    pub material_bind_group_layout: wgpu::BindGroupLayout,
    pub object_bind_group_layout: wgpu::BindGroupLayout,
}

/// Generic function to create a render pipeline from a material and definition
pub fn create_render_pipeline_from_def(
    device: &wgpu::Device,
    view_layout: &wgpu::BindGroupLayout, // Pass in the common @group(0)
    def: PipelineDefinition,
) -> CreatedPipeline {
    // process and parse the shader and ron file
    let material_source =
        fs::read_to_string(def.material_path).expect("Failed to read material file");
    let material_def: MaterialDefinition =
        ron::from_str(&material_source).expect("Failed to parse material definition");

    // TODO: validate the shader against the material_def here.

    // generate a layout for each bind group in the material definition
    let mut created_layouts: BTreeMap<u32, wgpu::BindGroupLayout> = BTreeMap::new();
    for (&group_index, layout_def) in &material_def.bind_group_layouts {
        let entries: Vec<wgpu::BindGroupLayoutEntry> = layout_def
            .bindings
            .iter()
            .map(|binding_def| create_layout_entry_from_metadata(binding_def))
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&format!(
                "{} Bind Group Layout @group({})",
                def.label, group_index
            )),
            entries: &entries,
        });
        created_layouts.insert(group_index, layout);
    }

    // assemble final pipeline layout
    let pipeline_bind_group_layouts_ref: Vec<&wgpu::BindGroupLayout> = std::iter::once(view_layout) // @group(0)
        .chain(created_layouts.values()) // @group(1), @group(2), etc
        .collect();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(&format!("{} Pipeline Layout", def.label)),
        bind_group_layouts: &pipeline_bind_group_layouts_ref,
        push_constant_ranges: &[],
    });

    // create the rendering pipeline
    let vs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(&format!("{} Vertex Shader", def.label)),
        source: def.vs_shader_source,
    });

    let fs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(&format!("{} Fragment Shader", def.label)),
        source: def.fs_shader_source,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(def.label),
        layout: Some(&pipeline_layout),
        cache: None,
        vertex: wgpu::VertexState {
            module: &vs_shader,
            entry_point: Some("vs_main"),
            buffers: def.vertex_buffers,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_shader,
            entry_point: Some("fs_main"),
            targets: def.fragment_targets,
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: def.depth_stencil,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    CreatedPipeline {
        pipeline,
        view_bind_group_layout: view_layout.clone(),
        material_bind_group_layout: created_layouts
            .remove(&1)
            .expect("Material missing @group(1)"),
        object_bind_group_layout: created_layouts
            .remove(&2)
            .expect("Object missing @group(2)"),
    }
}

/// A small helper to create a BindGroupLayoutEntry from BindingDef metadata
fn create_layout_entry_from_metadata(binding_def: &BindingDef) -> wgpu::BindGroupLayoutEntry {
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
                    "Storage" => wgpu::BufferBindingType::Storage { read_only: true },
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
