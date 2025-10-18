use crate::prelude::*;
use crate::render_world::global_extract::RenderWindowSizeResource;
use crate::render_world::graphics_context::resources::{
    RenderDevice, RenderSurface, RenderSurfaceConfig,
};
use bevy_ecs::prelude::*;

/// A system that reacts to window size changes and reconfigures the wgpu surface.
#[instrument(skip_all)]
pub fn reconfigure_wgpu_surface_system(
    // Input
    window_size: Res<RenderWindowSizeResource>,

    // Output
    surface: Res<RenderSurface>,
    device: Res<RenderDevice>,
    mut config: ResMut<RenderSurfaceConfig>,
) {
    // size of 0 is undefined in wgpu
    if window_size.width > 0.0 && window_size.height > 0.0 {
        debug!(
            target = "wgpu_resize",
            "Detected window resize. Reconfiguring wgpu surface to {}x{}",
            window_size.width,
            window_size.height
        );

        config.width = window_size.width as u32;
        config.height = window_size.height as u32;
        surface.configure(&device, &config);
    }
}
