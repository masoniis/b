use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderWindowSizeResource,
        graphics_context::resources::RenderQueue,
        passes::ui_pass::startup::{UiViewBuffer, UiViewData},
    },
};
use bevy_ecs::prelude::*;

/// A system that updates the orthographic projection matrix for the UI camera.
///
/// Should run if: the window size changes (view bind group layout should remain consistent).
#[instrument(skip_all)]
pub fn update_ui_view_system(
    // Input
    window_size: Res<RenderWindowSizeResource>,
    ui_view_buffer: Res<UiViewBuffer>,

    // Output (write a buffer)
    queue: Res<RenderQueue>,
) {
    debug!(
        target : "ui_efficiency",
        "Updating UI view data (this should only happen the screen was resized)..."
    );

    let projection_matrix =
        Mat4::orthographic_rh(0.0, window_size.width, window_size.height, 0.0, -1.0, 1.0);

    let ui_view_data = UiViewData {
        projection_matrix: projection_matrix.to_cols_array(),
    };

    queue.write_buffer(&ui_view_buffer.buffer, 0, bytemuck::bytes_of(&ui_view_data));
}
