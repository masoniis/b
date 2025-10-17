use crate::prelude::*;
use crate::render_world::{
    global_extract::RenderWindowSizeResource, resources::GraphicsContextResource,
};
use bevy_ecs::prelude::*;

// A system that reacts to window size changes and reconfigures the wgpu surface.
pub fn reconfigure_wgpu_surface_system(
    window_size: Res<RenderWindowSizeResource>,
    mut gfx: ResMut<GraphicsContextResource>,
) {
    // size of 0 is undefined in wgpu
    if window_size.width > 0.0 && window_size.height > 0.0 {
        debug!(
            target = "wgpu_resize",
            "Detected window resize. Reconfiguring wgpu surface to {}x{}",
            window_size.width,
            window_size.height
        );

        // Update the internal configuration with the new size.
        gfx.context.inform_resize(PhysicalSize {
            width: window_size.width as u32,
            height: window_size.height as u32,
        });
    }
}
