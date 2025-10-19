use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderCameraResource,
        graphics_context::resources::RenderQueue,
        passes::opaque_pass::startup::{OpaqueViewBuffer, OpaqueViewData},
    },
};
use bevy_ecs::prelude::*;

/// Takes the extracted camera data and uploads it to the GPU buffer for the Opaque Pass.
///
/// Since the camera is essentially constantly rechanging this needs to be run just about
/// every frame. I am not sure it is worth adding the optimization to only run on camera
/// updates as the write buffer here is pretty cheap.
#[instrument(skip_all)]
pub fn update_opaque_view_data_system(
    // Input
    camera_info: Res<RenderCameraResource>,
    view_buffer: Res<OpaqueViewBuffer>,

    // Output (writing buffer to queue)
    queue: Res<RenderQueue>,
) {
    let view_proj_matrix = camera_info.projection_matrix * camera_info.view_matrix;

    let camera_data = OpaqueViewData {
        view_proj_matrix: view_proj_matrix.to_cols_array(),
    };

    queue.write_buffer(&view_buffer.buffer, 0, bytemuck::cast_slice(&[camera_data]));
}
