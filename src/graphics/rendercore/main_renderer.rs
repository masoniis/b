use crate::graphics::rendercore::types::{WebGpuRenderer, DEPTH_FORMAT};
use crate::{
    ecs::resources::{
        asset_storage::MeshAsset, AssetStorageResource, CameraUniformResource, RenderQueueResource,
    },
    graphics::renderpass::render_pass::RenderPass,
};
use std::sync::Arc;

impl WebGpuRenderer {
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

            self.text_render_pass.viewport.update(
                &self.queue,
                glyphon::Resolution {
                    width: new_size.width,
                    height: new_size.height,
                },
            );
        }
    }

    pub fn render(
        &mut self,
        view: &wgpu::TextureView,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    ) -> Result<(), wgpu::SurfaceError> {
        self.scene_render_pass.prepare(
            &self.device,
            &self.queue,
            render_queue,
            mesh_assets,
            camera_uniform,
        );
        self.text_render_pass.prepare(
            &self.device,
            &self.queue,
            render_queue,
            mesh_assets,
            camera_uniform,
        );

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.scene_render_pass.render(
            &mut encoder,
            view,
            render_queue,
            mesh_assets,
            camera_uniform,
            &self.depth_texture_view,
            &self.camera_buffer,
            &self.instance_buffer,
            &self.render_pipeline,
            &self.camera_bind_group,
            &mut self.gpu_meshes,
        );
        self.text_render_pass.render(
            &mut encoder,
            view,
            render_queue,
            mesh_assets,
            camera_uniform,
            &self.depth_texture_view,
            &self.camera_buffer,
            &self.instance_buffer,
            &self.render_pipeline,
            &self.camera_bind_group,
            &mut self.gpu_meshes,
        );

        // Submit the command encoder containing both passes to the queue
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
