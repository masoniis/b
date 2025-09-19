use wgpu::{util::DeviceExt, Device, Queue, RenderPipeline};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x3, // color
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub struct QueuedDraw {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
    pub instance_count: u32,
}

#[derive(Resource)]
pub struct WebGpuRenderer {
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,

    draw_queue: Vec<QueuedDraw>,
}

use std::fs;
use bevy_ecs::prelude::Resource;

const SHADER_PATH: &str = "src/assets/shaders/scene/simple.wgsl";

impl WebGpuRenderer {
    pub fn new(device: Device, queue: Queue, config: &wgpu::SurfaceConfiguration) -> Self {
        // All logic related to surface, adapter, and creating the config
        // has been removed, as that is now handled by the App.

        let shader_source = fs::read_to_string(SHADER_PATH).expect("Failed to read shader file");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
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
                    // We correctly use the format from the config that was passed in.
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

        // The final struct no longer contains the surface, adapter, etc.
        Self {
            device,
            queue,
            render_pipeline,
            draw_queue: Vec::new(),
        }
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

            // Process all the prepared draw calls
            for draw in &self.draw_queue {
                let vertex_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Queued Draw Vertex Buffer"),
                            contents: bytemuck::cast_slice(&draw.vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                if let Some(indices) = &draw.indices {
                    let index_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("Queued Draw Index Buffer"),
                                contents: bytemuck::cast_slice(indices),
                                usage: wgpu::BufferUsages::INDEX,
                            });
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..indices.len() as u32, 0, 0..draw.instance_count);
                } else {
                    render_pass.draw(0..draw.vertices.len() as u32, 0..draw.instance_count);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

