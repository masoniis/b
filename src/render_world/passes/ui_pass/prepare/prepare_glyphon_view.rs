use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderWindowSizeResource,
        graphics_context::resources::RenderQueue,
        passes::ui_pass::gpu_resources::GlyphonViewportResource,
    },
};
use bevy_ecs::prelude::*;

/// A system that creates the orthographic projection matrix for the UI camera.
///
/// Run condition: If the window size or the view bind group layout changes.
#[instrument(skip_all)]
pub fn prepare_glyphon_view_system(
    // Input
    queue: Res<RenderQueue>,
    window_size: Res<RenderWindowSizeResource>,

    // Output (updated viewport)
    mut glyphon_viewport: ResMut<GlyphonViewportResource>,
) {
    debug!(
        target : "ui_efficiency",
        "Updating Glyphon viewport (this should only happen the screen was resized)..."
    );

    glyphon_viewport.0.update(
        &queue,
        glyphon::Resolution {
            width: window_size.width as u32,
            height: window_size.height as u32,
        },
    );
}
