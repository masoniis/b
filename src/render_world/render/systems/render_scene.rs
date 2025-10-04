use crate::prelude::*;
use crate::render_world::extract::RenderMeshStorageResource;
use crate::render_world::queue::resources::queue::RenderQueueResource;
use crate::render_world::{extract::RenderCameraResource, render::GraphicsContextResource};
use bevy_ecs::{prelude::ResMut, system::Res};
use wgpu::TextureViewDescriptor;

/// The main rendering system for when the game is running
pub fn render_scene_system(
    mut gfx_resource: ResMut<GraphicsContextResource>,
    camera_info: Res<RenderCameraResource>,
    queue: Res<RenderQueueResource>,
    render_mesh_storage: Res<RenderMeshStorageResource>,
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

    gfx.renderer
        .render(&view, &queue, &render_mesh_storage, &camera_info);

    output.present();
}
