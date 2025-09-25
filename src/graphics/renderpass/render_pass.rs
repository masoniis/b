use crate::{
    ecs::resources::{
        asset_storage::MeshAsset, AssetStorageResource, CameraUniformResource, RenderQueueResource,
    },
    graphics::rendercore::WebGpuRenderer,
};

pub trait RenderPass {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    );
    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        renderer: &mut WebGpuRenderer,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    );
}
