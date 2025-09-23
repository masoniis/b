use crate::graphics::{GpuMesh, Vertex};
use std::sync::Arc;
use wgpu::{util::DeviceExt, Device, Queue, RenderPipeline};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use glam::Mat4;
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update_view_proj(&mut self, proj: glam::Mat4) {
        self.view_proj = proj.to_cols_array_2d();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct TransformUniform {
    transform: [[f32; 4]; 4],
}

impl TransformUniform {
    fn new(transform: glam::Mat4) -> Self {
        Self {
            transform: transform.to_cols_array_2d(),
        }
    }
}

pub struct QueuedDraw {
    pub gpu_mesh: Arc<GpuMesh>,
    pub instance_count: u32,
    pub transform: glam::Mat4,
}

#[derive(Resource)]
pub struct WebGpuRenderer {
    // Core
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,

    // Public API
    draw_queue: Vec<QueuedDraw>,

    // Uniforms
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    transform_bind_group_layout: wgpu::BindGroupLayout,
}

use bevy_ecs::prelude::Resource;
use std::fs;

const SHADER_PATH: &str = "src/assets/shaders/scene/simple.wgsl";

impl WebGpuRenderer {
    pub fn new(device: Device, queue: Queue, config: &wgpu::SurfaceConfiguration) -> Self {
        let camera_uniform = CameraUniform::new();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader_source = fs::read_to_string(SHADER_PATH).expect("Failed to read shader file");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("transform_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &transform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            cache: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            device,
            queue,
            render_pipeline,
            draw_queue: Vec::new(),
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            transform_bind_group_layout,
        }
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn update_camera(&mut self, proj: glam::Mat4) {
        self.camera_uniform.update_view_proj(proj);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    /// Queue a draw call that the renderer pipeline will process during rendering phase.
    pub fn queue_draw(&mut self, draw: QueuedDraw) {
        self.draw_queue.push(draw);
    }

    /// Clear the current render queue. Should be used to clear queue before the next frame.
    pub fn clear_queue(&mut self) {
        self.draw_queue.clear();
    }

    pub fn render(&self, view: &wgpu::TextureView) -> Result<(), wgpu::SurfaceError> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0075,
                            g: 0.0125,
                            b: 0.0250,
                            a: 1.0000,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // Process all the prepared draw calls
            for draw in &self.draw_queue {
                let transform_uniform = TransformUniform::new(draw.transform);
                let transform_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Transform Buffer"),
                            contents: bytemuck::cast_slice(&[transform_uniform]),
                            usage: wgpu::BufferUsages::UNIFORM,
                        });

                let transform_bind_group =
                    self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &self.transform_bind_group_layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: transform_buffer.as_entire_binding(),
                        }],
                        label: Some("transform_bind_group"),
                    });

                render_pass.set_vertex_buffer(0, draw.gpu_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    draw.gpu_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.set_bind_group(1, &transform_bind_group, &[]);
                render_pass.draw_indexed(0..draw.gpu_mesh.index_count, 0, 0..draw.instance_count);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
