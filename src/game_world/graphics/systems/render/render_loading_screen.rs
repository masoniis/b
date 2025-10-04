// use crate::{
//     ecs_resources::{graphics_context::GraphicsContextResource, time::TimeResource},
//     prelude::*,
// };
// use bevy_ecs::prelude::{Res, ResMut};
// use wgpu::TextureViewDescriptor;
//
// /// The rendering system for the loading screen
// pub fn render_loading_screen_system(
//     mut gfx_resource: ResMut<GraphicsContextResource>,
//     time: Res<TimeResource>,
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
//     let view = output
//         .texture
//         .create_view(&TextureViewDescriptor::default());
//
//     gfx.renderer.render_loading_screen(&view, &time);
//
//     output.present();
// }
