use crate::{
    ecs::resources::{
        asset_storage::{AssetId, MeshAsset},
        AssetStorageResource, CameraUniformResource, RenderQueueResource,
    },
    graphics::{
        renderpass::{SceneRenderPass, SharedRenderData, TextRenderPass},
        GpuMesh,
    },
};
use std::{collections::HashMap, sync::Arc};

/// The types of renderpasses available in the engine.
pub enum RenderPass {
    Scene(SceneRenderPass),
    Text(TextRenderPass),
}

/// The context shared between all renderpasses
#[derive(Copy, Clone)]
pub struct RenderContext<'a> {
    pub view: &'a wgpu::TextureView,
    pub depth_texture_view: &'a wgpu::TextureView,
    pub shared_data: &'a SharedRenderData,
}

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
        context: RenderContext<'a>,
        ecs_render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        instance_buffer: &wgpu::Buffer,
        gpu_meshes: &mut HashMap<AssetId, Arc<GpuMesh>>,
        render_pipeline: &wgpu::RenderPipeline,
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

    fn render<'a>(&'a self, encoder: &mut wgpu::CommandEncoder, context: RenderContext<'a>);
}
