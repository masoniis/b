use crate::{
    prelude::*,
    render_world::{
        extract::RenderTimeResource,
        // prepare::{
        //     prepare_pipelines::LOADING_SCREEN_PIPELINE_ID, LoadingScreenPipelineLayoutsResource,
        //     PipelineCacheResource,
        // },
        render::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::{Res, ResMut};
use wgpu::TextureViewDescriptor;

// The rendering system for the loading screen
pub fn render_loading_screen_system(
    mut gfx_resource: ResMut<GraphicsContextResource>,
    time: Res<RenderTimeResource>,
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

    gfx.renderer.render_loading_screen(&view, &time);

    output.present();
}

// pub fn render_loading_screen_system(
//     mut gfx_resource: ResMut<GraphicsContextResource>,
//     pipeline_cache: Res<PipelineCacheResource>,
//     layouts: Res<LoadingScreenPipelineLayoutsResource>
// ) {
//     let gfx = &mut gfx_resource.context;
//
//     let output = match gfx.surface.get_current_texture() {
//         Ok(texture) => texture,
//         Err(wgpu::SurfaceError::Lost) => {
//             warn!("WGPU SurfaceError::Lost, surface will be reconfigured automatically on next frame.");
//             return;
//         }
//         Err(wgpu::SurfaceError::OutOfMemory) => {
//             error!("WGPU SurfaceError::OutOfMemory, this is fatal.");
//             // TODO: Send an AppExit event or transition to a closing state.
//             // For now, we must stop rendering.
//             return;
//         }
//         Err(e) => {
//             error!("Error acquiring surface texture: {:?}", e);
//             return;
//         }
//     };
//
//     let view = output.texture.create_view(&Default::default());
//
//     let mut encoder = gfx.device.create_command_encoder(...);
//     let pipeline = pipeline_cache.get(LOADING_SCREEN_PIPELINE_ID).unwrap();
//
//     let mut render_pass = encoder.begin_render_pass(...);
//     render_pass.set_pipeline(pipeline);
//     render_pass.set_bind_group(0, &layouts.time_layout, &[]);
//     render_pass.draw(0..4, 0..1);
//     drop(render_pass);
//
//     gfx.queue.submit(std::iter::once(encoder.finish()));
//     output.present();
// }
