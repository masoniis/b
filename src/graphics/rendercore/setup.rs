use super::types::{InstanceRaw, WebGpuRenderer, DEPTH_FORMAT, MAX_TRANSFORMS, SHADER_PATH};
use crate::graphics::{
    renderpass::{shared_data::SharedRenderData, RenderPass},
    SceneRenderPass, TextRenderPass, Vertex,
};
use std::{collections::HashMap, fs, sync::Arc};
use winit::dpi::PhysicalSize;

impl WebGpuRenderer {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let shared_data = SharedRenderData::new(&device);

        let shader_source = fs::read_to_string(SHADER_PATH).expect("Failed to read shader file");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (MAX_TRANSFORMS * std::mem::size_of::<InstanceRaw>() as u64),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[DEPTH_FORMAT],
        });
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&shared_data.camera_bind_group_layout], // Use the shared layout
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            cache: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // Standard depth comparison
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let text_render_pass = TextRenderPass::new(
            &device,
            &queue,
            config.format,
            PhysicalSize::new(config.width, config.height),
        );
        let scene_render_pass = SceneRenderPass::new(device.clone(), queue.clone());

        let passes = vec![
            RenderPass::Scene(scene_render_pass),
            RenderPass::Text(text_render_pass),
        ];

        Self {
            device,
            queue,
            render_pipeline,
            instance_buffer,
            depth_texture_view,
            gpu_meshes: HashMap::new(),
            passes,
            shared_data,
        }
    }
}
