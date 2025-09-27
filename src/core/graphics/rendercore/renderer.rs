use crate::{
    core::graphics::{
        renderpass::{
            traits::{ISceneRenderPass, ITextRenderPass},
            RenderPass, RenderPassContex, SharedRenderData,
        },
        types::mesh::GpuMesh,
    },
    ecs_resources::{
        asset_storage::{AssetId, MeshAsset},
        AssetStorageResource, CameraUniformResource, RenderQueueResource,
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

    // Render Passes
    pub passes: Vec<RenderPass>,

    // Shared Data
    pub shared_data: SharedRenderData,

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
    ) -> Result<(), wgpu::SurfaceError> {
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
                    );
                }
                RenderPass::Text(ref mut text_pass) => {
                    ITextRenderPass::render(text_pass, &mut encoder, context);
                }
            }
        }

        // Submit the command encoder containing all the accumulated passes to the queue
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
