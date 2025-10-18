use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderWindowSizeResource,
        passes::ui_pass::startup::{UiViewBuffer, UiViewData},
        resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;

/// A system that updates the orthographic projection matrix for the UI camera.
///
/// Run condition: If the window size changes (view bind group layout should remain consistent).
#[instrument(skip_all)]
pub fn update_ui_view_system(
    // Input
    gfx: Res<GraphicsContextResource>,
    window_size: Res<RenderWindowSizeResource>,
    ui_view_buffer: Res<UiViewBuffer>,
) {
    debug!(
        target : "ui_efficiency",
        "Updating UI view data (this should only happen the screen was resized)..."
    );

    let queue = &gfx.context.queue;

    let projection_matrix =
        Mat4::orthographic_rh(0.0, window_size.width, window_size.height, 0.0, -1.0, 1.0);

    let ui_view_data = UiViewData {
        projection_matrix: projection_matrix.to_cols_array(),
    };

    queue.write_buffer(&ui_view_buffer.buffer, 0, bytemuck::bytes_of(&ui_view_data));
}
