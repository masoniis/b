use crate::{
    prelude::*,
    render_world::uniforms::CameraUniform,
    render_world::{
        global_extract::resources::RenderCameraResource,
        passes::opaque_pass::prepare::resources::bind_groups::ViewBindGroup,
        resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;

/// Takes the extracted camera data and uploads it to the GPU buffer for the ViewBindGroup.
#[instrument(skip_all)]
pub fn prepare_view_bind_group_system(
    // Input
    camera_info: Res<RenderCameraResource>,
    view_bind_group: Res<ViewBindGroup>, // We get the buffer from this resource
    gfx_context: Res<GraphicsContextResource>,
) {
    let view_proj_matrix = camera_info.projection_matrix * camera_info.view_matrix;

    // Create the GPU-compatible uniform data struct
    let camera_uniform = CameraUniform {
        view_proj: view_proj_matrix.to_cols_array_2d(),
    };

    // Update the buffer on the GPU with the new matrix data for this frame.
    gfx_context.context.queue.write_buffer(
        &view_bind_group.buffer,
        0,
        bytemuck::cast_slice(&[camera_uniform]),
    );
}
