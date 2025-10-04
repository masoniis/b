use crate::{
    // game_world::graphics::resources::{
    //     queue::RenderQueueResource, uniforms::CameraUniformResource,
    // },
    prelude::*,
    render_world::pipeline::GraphicsContextResource,
};

use bevy_ecs::prelude::ResMut;
use wgpu::TextureViewDescriptor;

/// The main rendering system for when the game is running
pub fn render_scene_system(
    mut gfx_resource: ResMut<GraphicsContextResource>,
    // render_queue: Res<RenderQueueResource>,
    // mesh_assets: Res<AssetStorageResource<MeshAsset>>,
    // camera_uniform: Res<CameraUniformResource>,
) {
    let gfx = &mut gfx_resource.context;

    let output = match gfx.surface.get_current_texture() {
        Ok(texture) => texture,
        Err(wgpu::SurfaceError::Lost) => {
            warn!("WGPU SurfaceError::Lost, surface will be reconfigured automatically on next frame.");
            return;
        }
        Err(wgpu::SurfaceError::OutOfMemory) => {
            error!("WGPU SurfaceError::OutOfMemory, this is fatal.");
            // TODO: Send an AppExit event or transition to a closing state.
            // For now, we must stop rendering.
            return;
        }
        Err(e) => {
            error!("Error acquiring surface texture: {:?}", e);
            return;
        }
    };

    let view = output
        .texture
        .create_view(&TextureViewDescriptor::default());

    // gfx.renderer
    //     .render(&view, &render_queue, &mesh_assets, &camera_uniform);

    output.present();
}
