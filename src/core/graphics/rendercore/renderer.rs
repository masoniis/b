use crate::{
    core::graphics::{
        rendercore::time_uniform::TimeUniform,
        renderpass::{
            traits::{ISceneRenderPass, ITextRenderPass},
            RenderPass, RenderPassContex, SharedRenderData,
        },
        types::mesh::GpuMesh,
    },
    ecs_modules::graphics::{CameraUniformResource, RenderQueueResource},
    ecs_resources::{
        asset_storage::{AssetId, MeshAsset},
        time::TimeResource,
        AssetStorageResource,
    },
};
use std::{collections::HashMap, sync::Arc};
use wgpu::{Device, Queue, RenderPipeline};

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct Renderer {
    // Core
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub render_pipeline: RenderPipeline,
    pub loading_screen_pipeline: RenderPipeline,

    // Render Passes
    pub passes: Vec<RenderPass>,

    // Shared Data
    pub shared_data: SharedRenderData,
    pub texture_bind_group: wgpu::BindGroup,

    // Buffers
    pub depth_texture_view: wgpu::TextureView,
    pub instance_buffer: wgpu::Buffer,

    // Assets
    pub gpu_meshes: HashMap<AssetId, Arc<GpuMesh>>,
}

impl Renderer {
    pub fn get_device(&self) -> Arc<wgpu::Device> {
        self.device.clone()
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
                format: DEPTH_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[DEPTH_FORMAT],
            });

            self.depth_texture_view =
                depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }

        // Glyphon requires updated resolution on resize
        for pass in &mut self.passes {
            if let RenderPass::Text(text_pass) = pass {
                text_pass.viewport.update(
                    &self.queue,
                    glyphon::Resolution {
                        width: new_size.width,
                        height: new_size.height,
                    },
                );
            }
        }
    }

    pub fn render(
        &mut self,
        view: &wgpu::TextureView,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    ) {
        self.shared_data
            .update_camera(&self.queue, camera_uniform.view_proj_matrix);

        for pass in &mut self.passes {
            match pass {
                RenderPass::Scene(ref mut scene_pass) => {
                    ISceneRenderPass::prepare(
                        scene_pass,
                        &self.device,
                        &self.queue,
                        render_queue,
                        mesh_assets,
                        camera_uniform,
                    );
                }
                RenderPass::Text(ref mut text_pass) => {
                    ITextRenderPass::prepare(
                        text_pass,
                        &self.device,
                        &self.queue,
                        render_queue,
                        mesh_assets,
                        camera_uniform,
                    );
                }
            }
        }

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let context = RenderPassContex {
            view,
            depth_texture_view: &self.depth_texture_view,
            shared_data: &self.shared_data,
        };

        for pass in &mut self.passes {
            match pass {
                RenderPass::Scene(ref mut scene_pass) => {
                    ISceneRenderPass::render(
                        scene_pass,
                        &mut encoder,
                        context,
                        render_queue,
                        mesh_assets,
                        &self.instance_buffer,
                        &mut self.gpu_meshes,
                        &self.render_pipeline,
                        &self.texture_bind_group,
                    );
                }
                RenderPass::Text(ref mut text_pass) => {
                    ITextRenderPass::render(text_pass, &mut encoder, context);
                }
            }
        }

        // Submit the command encoder containing all the accumulated passes to the queue
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    /// Render a shader-based loading screen
    pub fn render_loading_screen(&mut self, view: &wgpu::TextureView, time: &TimeResource) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Loading Screen Encoder"),
            });

        let mut time_uniform = TimeUniform::new();
        time_uniform.update_total_time(time.total_elapse.as_secs_f32());
        self.queue.write_buffer(
            &self.shared_data.time_buffer,
            0,
            bytemuck::cast_slice(&[time_uniform]),
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Loading Screen Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.loading_screen_pipeline);
            render_pass.set_bind_group(0, &self.shared_data.time_bind_group, &[]);
            render_pass.draw(0..4, 0..1); // 4 vertices for 4 corners
        }

        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
