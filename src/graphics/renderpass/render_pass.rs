use crate::{
    ecs::resources::{
        asset_storage::{AssetId, MeshAsset},
        AssetStorageResource, CameraUniformResource, RenderQueueResource,
    },
    graphics::GpuMesh,
};
use std::{collections::HashMap, sync::Arc};

pub trait RenderPass {
    /// Method to run just before the rendering phase begins.
    /// Necessary for some renderpasses that require preprocessing
    /// and can be used to create buffers as well.
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    );

    /// Method to implement the renderpass itself.
    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
        depth_texture_view: &wgpu::TextureView,
        camera_buffer: &wgpu::Buffer,
        instance_buffer: &wgpu::Buffer,
        render_pipeline: &wgpu::RenderPipeline,
        camera_bind_group: &wgpu::BindGroup,
        gpu_meshes: &mut HashMap<AssetId, Arc<GpuMesh>>,
    );
}
