use crate::{
    ecs_modules::rendering::resources::{
        queue::RenderQueueResource, uniforms::CameraUniformResource,
    },
    ecs_resources::{
        asset_storage::{AssetStorageResource, MeshAsset},
        graphics_context::GraphicsContextResource,
    },
    prelude::*,
};
use bevy_ecs::prelude::{Res, ResMut};

// The main rendering system for when the game is running
pub fn render_system(
    mut gfx_resource: ResMut<GraphicsContextResource>,
    render_queue: Res<RenderQueueResource>,
    mesh_assets: Res<AssetStorageResource<MeshAsset>>,
    camera_uniform: Res<CameraUniformResource>,
) {
    let gfx = &mut gfx_resource.context;

    // The render call is now self-contained within the system.
    // Error handling for wgpu::SurfaceError::Lost/OutOfMemory can happen here,
    // though you might need to send an event to the main loop to handle exiting.
    match gfx.render(&render_queue, &mesh_assets, &camera_uniform) {
        Ok(_) => {}
        Err(wgpu::SurfaceError::Lost) => {
            warn!("WGPU SurfaceError::Lost, surface will be reconfigured automatically on next frame.");
            // You might need to resize the surface here if the window size changed.
            // This requires access to the winit window, which can also be a resource.
        }
        Err(wgpu::SurfaceError::OutOfMemory) => {
            error!("WGPU SurfaceError::OutOfMemory, this is fatal.");
            // Consider sending an AppExit event
        }
        Err(e) => eprintln!("Error during render: {:?}", e),
    }
}

// The rendering system for the loading screen
pub fn render_loading_system(mut gfx_resource: ResMut<GraphicsContextResource>) {
    let gfx = &mut gfx_resource.context;

    match gfx.render_loading_screen() {
        Ok(_) => {}
        Err(wgpu::SurfaceError::Lost) => {
            warn!("WGPU SurfaceError::Lost, surface will be reconfigured automatically on next frame.");
        }
        Err(wgpu::SurfaceError::OutOfMemory) => {
            error!("WGPU SurfaceError::OutOfMemory, this is fatal.");
        }
        Err(e) => eprintln!("Error during render: {:?}", e),
    }
}
