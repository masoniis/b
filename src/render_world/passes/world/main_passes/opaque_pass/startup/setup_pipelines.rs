use crate::render_world::graphics_context::resources::{RenderDevice, RenderSurfaceConfig};
use crate::render_world::passes::world::gpu_resources::world_uniforms::ChunkStorageBindGroupLayout;
use crate::render_world::passes::world::main_passes::shared_resources::main_depth_texture::MAIN_DEPTH_FORMAT;
use crate::render_world::passes::world::main_passes::shared_resources::{
    CentralCameraViewBindGroupLayout, EnvironmentBindGroupLayout, TextureArrayBindGroupLayout,
};
use bevy_ecs::prelude::*;
use tracing::instrument;
use wesl::include_wesl;

/// A resource that holds all the opaque phase pipelines.
#[derive(Resource)]
pub struct OpaquePipelines {
    /// A pipeline that draws filled opaque geometry.
    pub fill: wgpu::RenderPipeline,

    /// A pipeline that draws wireframe opaque geometry.
    pub wireframe: wgpu::RenderPipeline,

    /// A pipeline that draws the skybox.
    pub skybox: wgpu::RenderPipeline,
}

impl FromWorld for OpaquePipelines {
    #[instrument(skip_all)]
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let config = world.resource::<RenderSurfaceConfig>();

        let view_layout = world.resource::<CentralCameraViewBindGroupLayout>();
        let environment_layout = world.resource::<EnvironmentBindGroupLayout>();
        let texture_layout = world.resource::<TextureArrayBindGroupLayout>();
        let chunk_storage_layout = world.resource::<ChunkStorageBindGroupLayout>();

        let opaque_fragment_target = [Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let opaque_depth_stencil = Some(wgpu::DepthStencilState {
            format: MAIN_DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::GreaterEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });

        // INFO: ---------------------------------
        //         regular opaque pipeline
        // ---------------------------------------

        let fill_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Opaque Pipeline Layout"),
            bind_group_layouts: &[
                &view_layout.0,
                &environment_layout.0,
                &texture_layout.0,
                &chunk_storage_layout.0,
            ],
            push_constant_ranges: &[],
        });

        let vs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Opaque Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("opaque_vert").into()),
        });

        let fs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Opaque Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("opaque_frag").into()),
        });

        let fill_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Opaque Pipeline"),
            layout: Some(&fill_pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &vs_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_shader,
                entry_point: Some("fs_main"),
                targets: &opaque_fragment_target,
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                polygon_mode: wgpu::PolygonMode::Fill,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: opaque_depth_stencil.clone(),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // INFO: -----------------------------------
        //         wireframe opaque pipeline
        // -----------------------------------------

        let wireframe_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Wireframe Opaque Pipeline Layout"),
                bind_group_layouts: &[
                    &view_layout.0,
                    &environment_layout.0,
                    &texture_layout.0,
                    &chunk_storage_layout.0,
                ],
                push_constant_ranges: &[],
            });

        let wireframe_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Wireframe Opaque Pipeline"),
            layout: Some(&wireframe_pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &vs_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_shader,
                entry_point: Some("fs_main"),
                targets: &opaque_fragment_target,
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                polygon_mode: wgpu::PolygonMode::Line,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: opaque_depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // INFO: --------------------------------
        //         skybox opaque pipeline
        // --------------------------------------

        let skybox_depth_stencil = Some(wgpu::DepthStencilState {
            format: MAIN_DEPTH_FORMAT,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::GreaterEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });

        let skybox_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Skybox Pipeline Layout"),
                bind_group_layouts: &[&view_layout.0, &environment_layout.0],
                push_constant_ranges: &[],
            });

        let skybox_vs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skybox Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("skybox_vert").into()),
        });

        let skybox_fs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skybox Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("skybox_frag").into()),
        });

        let skybox_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skybox Opaque Pipeline"),
            layout: Some(&skybox_pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &skybox_vs_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &skybox_fs_shader,
                entry_point: Some("fs_main"),
                targets: &opaque_fragment_target,
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                polygon_mode: wgpu::PolygonMode::Fill,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: skybox_depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            fill: fill_pipeline,
            wireframe: wireframe_pipeline,
            skybox: skybox_pipeline,
        }
    }
}

/// A resource that defines the current opaque render mode
#[derive(Resource, Default, Debug, PartialEq)]
pub enum OpaqueRenderMode {
    #[default]
    Fill,
    Wireframe,
}
