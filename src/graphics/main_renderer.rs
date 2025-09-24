use crate::graphics::{GlyphonRenderer, GpuMesh, Vertex};
use std::sync::Arc;
use tracing::warn;
use wgpu::{util::DeviceExt, Device, Queue, RenderPipeline};

use glam::Mat4;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update_view_proj(&mut self, proj: glam::Mat4) {
        self.view_proj = proj.to_cols_array_2d();
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model_matrix: [[f32; 4]; 4],
}

impl InstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2, // model_row_0
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3, // model_row_1
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 4]>() * 2) as wgpu::BufferAddress,
                    shader_location: 4, // model_row_2
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 4]>() * 3) as wgpu::BufferAddress,
                    shader_location: 5, // model_row_3
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
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
    device: Arc<Device>,
    queue: Arc<Queue>,
    render_pipeline: RenderPipeline,

    // Buffers
    depth_texture_view: wgpu::TextureView,
    instance_buffer: wgpu::Buffer,

    // Public API
    draw_queue: Vec<QueuedDraw>,

    // Uniforms
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
}

use bevy_ecs::prelude::Resource;
use std::fs;

const SHADER_PATH: &str = "src/assets/shaders/scene/simple.wgsl";
const MAX_TRANSFORMS: u64 = 100000;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

impl WebGpuRenderer {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
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
                bind_group_layouts: &[&camera_bind_group_layout], // Removed transform_bind_group_layout
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            cache: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), InstanceRaw::desc()], // Added InstanceRaw::desc()
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

        Self {
            device,
            queue,
            render_pipeline,
            draw_queue: Vec::new(),
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            instance_buffer,
            depth_texture_view,
        }
    }

    pub fn get_device(&self) -> Arc<Device> {
        self.device.clone()
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

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: new_size.width,
                    height: new_size.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH_FORMAT, // The const we added earlier
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[DEPTH_FORMAT],
            });
            self.depth_texture_view =
                depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    pub fn render(
        &mut self,
        view: &wgpu::TextureView,
        text_renderer: &mut GlyphonRenderer,
    ) -> Result<(), wgpu::SurfaceError> {
        // --- Instance Buffer Preparation (same as before) ---
        let num_queued_draws = self.draw_queue.len();
        if num_queued_draws > MAX_TRANSFORMS as usize {
            warn!(
            "Number of queued draws ({}) exceeds MAX_TRANSFORMS ({}). Only rendering the first {} transforms.",
            num_queued_draws, MAX_TRANSFORMS, MAX_TRANSFORMS
        );
        }

        let mut instances = Vec::with_capacity(num_queued_draws.min(MAX_TRANSFORMS as usize));
        for draw in self.draw_queue.iter().take(MAX_TRANSFORMS as usize) {
            instances.push(InstanceRaw {
                model_matrix: draw.transform.to_cols_array_2d(),
            });
        }

        self.queue
            .write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // --- Pass 1: Render the 3D Scene ---
        {
            let mut scene_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Scene Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Clear the screen with the background color
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0075,
                            g: 0.0125,
                            b: 0.0250,
                            a: 1.0000,
                        }),
                        // Store the result to be used by the next pass
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                // Use the depth buffer
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), // Clear depth to 1.0
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            scene_pass.set_pipeline(&self.render_pipeline);
            scene_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            scene_pass.set_vertex_buffer(0, self.draw_queue[0].gpu_mesh.vertex_buffer.slice(..));
            scene_pass.set_index_buffer(
                self.draw_queue[0].gpu_mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            scene_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            scene_pass.draw_indexed(
                0..self.draw_queue[0].gpu_mesh.index_count,
                0,
                0..instances.len() as u32,
            );
        } // The `scene_pass` is dropped here, ending the first render pass

        // --- Pass 2: Render the Text Overlay ---
        {
            let mut text_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Text Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // LOAD the contents of the previous pass
                        load: wgpu::LoadOp::Load,
                        // Store the results of this pass (the text on top of the scene)
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                // No depth buffer for the UI pass
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            text_renderer.render(&mut text_pass).unwrap();
        } // The `text_pass` is dropped here, ending the second render pass

        // Submit the command encoder containing both passes to the queue
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
