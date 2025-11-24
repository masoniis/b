use crate::{
    prelude::*,
    render_world::{
        graphics_context::resources::{RenderDevice, RenderSurfaceConfig},
        passes::ui_pass::gpu_resources::{
            material_binding::UiMaterialBindGroupLayout, object_binding::UiObjectBindGroupLayout,
            view_binding::UiViewBindGroupLayout,
        },
    },
};
use bevy_ecs::prelude::*;
use wesl::include_wesl;

/// A resource to hold the pipeline and bind group layouts for our UI shader.
#[derive(Resource, Deref, DerefMut)]
pub struct UiPipeline {
    inner: wgpu::RenderPipeline,
}

impl FromWorld for UiPipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let config = world.resource::<RenderSurfaceConfig>();

        let view_layout = world.resource::<UiViewBindGroupLayout>();
        let material_layout = world.resource::<UiMaterialBindGroupLayout>();
        let object_layout = world.resource::<UiObjectBindGroupLayout>();

        const UI_VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
            array_stride: (2 * std::mem::size_of::<f32>()) as wgpu::BufferAddress, // only 2d points for ui
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        };

        // define the specific fragment target for UI (with alpha blending)
        let ui_fragment_target = [Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("UI Pipeline Layout"),
            bind_group_layouts: &[&view_layout.0, &material_layout.0, &object_layout.0],
            push_constant_ranges: &[],
        });

        let vs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("UI Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("ui_vert").into()),
        });

        let fs_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("UI Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_wesl!("ui_frag").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("UI Pipeline"),
            layout: Some(&pipeline_layout),
            cache: None,
            vertex: wgpu::VertexState {
                module: &vs_shader,
                entry_point: Some("vs_main"),
                buffers: &[UI_VERTEX_BUFFER_LAYOUT],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_shader,
                entry_point: Some("fs_main"),
                targets: &ui_fragment_target,
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self { inner: pipeline }
    }
}
