use crate::{
    core::graphics::renderpass::RenderPassContex,
    render_world::{extract::RenderMeshStorageResource, queue::RenderQueueResource},
};

/// A trait for main scene renderpasses
pub trait ISceneRenderPass {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_queue: &RenderQueueResource,
        render_mesh_storage: &RenderMeshStorageResource,
        camera_uniform: &crate::render_world::extract::RenderCameraResource,
    );

    fn render<'a>(
        &'a self,
        encoder: &mut wgpu::CommandEncoder,
        context: RenderPassContex<'a>,
        ecs_render_queue: &RenderQueueResource,
        render_mesh_storage: &RenderMeshStorageResource,
        instance_buffer: &wgpu::Buffer,
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
        render_mesh_storage: &RenderMeshStorageResource,
        camera_uniform: &crate::render_world::extract::RenderCameraResource,
    );

    fn render<'a>(&'a self, encoder: &mut wgpu::CommandEncoder, context: RenderPassContex<'a>);
}
