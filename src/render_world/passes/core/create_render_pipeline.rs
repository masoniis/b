use crate::render_world::passes::core::{BindingDef, MaterialDefinition};
use std::collections::BTreeMap;
use std::fs;

pub struct PipelineDefinition<'a> {
    pub label: &'a str,
    pub material_path: &'a str,
    pub vs_shader_source: wgpu::ShaderSource<'a>,
    pub fs_shader_source: Option<wgpu::ShaderSource<'a>>,
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub fragment_targets: &'a [Option<wgpu::ColorTargetState>],
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    pub primitive: wgpu::PrimitiveState,
}

pub struct CreatedPipeline {
    pub pipeline: wgpu::RenderPipeline,
    bind_group_layouts: BTreeMap<u32, wgpu::BindGroupLayout>,
}

impl CreatedPipeline {
    /// Get a reference to a created bind group layout by its group index
    pub fn get_layout(&self, group_index: u32) -> &wgpu::BindGroupLayout {
        self.bind_group_layouts.get(&group_index).expect(&format!(
            "Bind group layout for @group({}) not found in CreatedPipeline",
            group_index
        ))
    }
}

/// Generic function to create a render pipeline from a material and definition
pub fn create_render_pipeline_from_def(
    device: &wgpu::Device,
    shared_layouts: &[&wgpu::BindGroupLayout],
    pipeline_def: PipelineDefinition,
) -> CreatedPipeline {
    // process and parse the shader and ron file
    let material_source =
        fs::read_to_string(pipeline_def.material_path).expect("Failed to read material file");
    let material_def: MaterialDefinition =
        ron::from_str(&material_source).expect("Failed to parse material definition");

    // TODO: validate the shader against the material_def here.

    // INFO: ---------------------------------------------------------------
    //        generate a layout for each bind group in material def
    // ---------------------------------------------------------------------

    let mut created_layouts: BTreeMap<u32, wgpu::BindGroupLayout> = BTreeMap::new();
    for (&group_index, layout_def) in &material_def.bind_group_layouts {
        if (group_index as usize) < shared_layouts.len() {
            panic!(
                "Material '{}' tries to define @group({}), which is reserved by a shared layout.
                (Shared layouts occupy groups 0..{}).
                Please start the material-defined groups at @group({}).",
                pipeline_def.material_path,
                group_index,
                shared_layouts.len(),
                shared_layouts.len()
            );
        }

        let entries: Vec<wgpu::BindGroupLayoutEntry> = layout_def
            .bindings
            .iter()
            .map(|binding_def| create_layout_entry_from_metadata(binding_def))
            .collect();
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&format!(
                "{} Bind Group Layout @group({})",
                pipeline_def.label, group_index
            )),
            entries: &entries,
        });
        created_layouts.insert(group_index, layout);
    }

    // get a reference to the newly created RON layouts, sorted by key
    let ron_layouts_ref: Vec<&wgpu::BindGroupLayout> = created_layouts.values().collect();

    // combine all parsed layouts into a single pipeline layout
    let pipeline_bind_group_layouts_ref: Vec<&wgpu::BindGroupLayout> = shared_layouts
        .iter()
        .copied()
        .chain(ron_layouts_ref.into_iter())
        .collect();

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(&format!("{} Pipeline Layout", pipeline_def.label)),
        bind_group_layouts: &pipeline_bind_group_layouts_ref,
        push_constant_ranges: &[],
    });

    // INFO: --------------------------------------
    //        construct rendering pipeline
    // --------------------------------------------

    let vs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(&format!("{} Vertex Shader", pipeline_def.label)),
        source: pipeline_def.vs_shader_source,
    });

    let fs_shader = if let Some(fs_source) = pipeline_def.fs_shader_source {
        Some(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{} Fragment Shader", pipeline_def.label)),
            source: fs_source,
        }))
    } else {
        None
    };

    let fragment_state = if let Some(ref fs_shader) = fs_shader {
        Some(wgpu::FragmentState {
            module: fs_shader,
            entry_point: Some("fs_main"),
            targets: pipeline_def.fragment_targets,
            compilation_options: Default::default(),
        })
    } else {
        None
    };

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(pipeline_def.label),
        layout: Some(&pipeline_layout),
        cache: None,
        vertex: wgpu::VertexState {
            module: &vs_shader,
            entry_point: Some("vs_main"),
            buffers: pipeline_def.vertex_buffers,
            compilation_options: Default::default(),
        },
        fragment: fragment_state,
        primitive: pipeline_def.primitive,
        depth_stencil: pipeline_def.depth_stencil,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    CreatedPipeline {
        pipeline,
        bind_group_layouts: created_layouts,
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
        "Texture" => {
            let opts = binding_def
                .texture_options
                .as_ref()
                .expect("Texture must have texture_options");

            wgpu::BindingType::Texture {
                sample_type: match opts.sample_type.as_str() {
                    "Float" => wgpu::TextureSampleType::Float { filterable: true },
                    "Depth" => wgpu::TextureSampleType::Depth,
                    "Sint" => wgpu::TextureSampleType::Sint,
                    "Uint" => wgpu::TextureSampleType::Uint,
                    _ => panic!("Unknown texture sample type"),
                },

                view_dimension: match opts.view_dimension.as_str() {
                    "1d" => wgpu::TextureViewDimension::D1,
                    "2d" => wgpu::TextureViewDimension::D2,
                    "2d_array" => wgpu::TextureViewDimension::D2Array,
                    "cube" => wgpu::TextureViewDimension::Cube,
                    "cube_array" => wgpu::TextureViewDimension::CubeArray,
                    "3d" => wgpu::TextureViewDimension::D3,
                    _ => panic!("Unknown texture view dimension"),
                },

                multisampled: opts.multisampled,
            }
        }
        "Sampler" => {
            let opts = binding_def
                .sampler_options
                .as_ref()
                .expect("Sampler must have sampler_options");

            wgpu::BindingType::Sampler(match opts.ty.as_str() {
                "Filtering" => wgpu::SamplerBindingType::Filtering,

                "NonFiltering" => wgpu::SamplerBindingType::NonFiltering,

                "Comparison" => wgpu::SamplerBindingType::Comparison,

                _ => panic!("Unknown sampler type"),
            })
        }
        _ => panic!("Unsupported binding type"),
    };

    wgpu::BindGroupLayoutEntry {
        binding: binding_def.binding,
        visibility,
        ty,
        count: None,
    }
}
