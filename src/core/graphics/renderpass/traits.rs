use crate::{
    core::graphics::renderpass::RenderPassContex,
    core::graphics::types::GpuMesh,
    ecs_modules::rendering::{CameraUniformResource, RenderQueueResource},
    ecs_resources::{
        asset_storage::{AssetId, MeshAsset},
        AssetStorageResource,
    },
};
use std::{collections::HashMap, sync::Arc};

/// A trait for main scene renderpasses
pub trait ISceneRenderPass {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    );

    fn render<'a>(
        &'a self,
        encoder: &mut wgpu::CommandEncoder,
        context: RenderPassContex<'a>,
        ecs_render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        instance_buffer: &wgpu::Buffer,
        gpu_meshes: &mut HashMap<AssetId, Arc<GpuMesh>>,
        render_pipeline: &wgpu::RenderPipeline,
        texture_bind_group: &wgpu::BindGroup,
    );
}

/// A trait for text-only glyphon powered renderpasses.
pub trait ITextRenderPass {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    );

    fn render<'a>(&'a self, encoder: &mut wgpu::CommandEncoder, context: RenderPassContex<'a>);
}
