use crate::core::graphics::renderpass::{
    scene_pass::SceneRenderPass, shared_data::SharedRenderData, text_pass::TextRenderPass,
};

/// The types of renderpasses available in the engine.
pub enum RenderPass {
    Scene(SceneRenderPass),
    Text(TextRenderPass),
}

/// The context shared between all renderpasses
#[derive(Copy, Clone)]
pub struct RenderPassContex<'a> {
    pub view: &'a wgpu::TextureView,
    pub depth_texture_view: &'a wgpu::TextureView,
    pub shared_data: &'a SharedRenderData,
}
