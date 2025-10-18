use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderCameraResource, graphics_context::resources::RenderQueue,
        passes::opaque_pass::prepare::resources::bind_groups::ViewBindGroup,
        uniforms::CameraUniform,
    },
};
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct OpaquePassViewBindGroup {
    pub bind_group: wgpu::BindGroup,
}

/// Takes the extracted camera data and uploads it to the GPU buffer for the ViewBindGroup.
#[instrument(skip_all)]
pub fn prepare_view_bind_group_system(
    // Input
    queue: Res<RenderQueue>,
    camera_info: Res<RenderCameraResource>,
    view_bind_group: Res<ViewBindGroup>, // buffer from this resource
) {
    let view_proj_matrix = camera_info.projection_matrix * camera_info.view_matrix;

    // Create the GPU-compatible uniform data struct
    let camera_uniform = CameraUniform {
        view_proj: view_proj_matrix.to_cols_array_2d(),
    };

    // Update the buffer on the GPU with the new matrix data for this frame.
    queue.0.write_buffer(
        &view_bind_group.buffer,
        0,
        bytemuck::cast_slice(&[camera_uniform]),
    );
}
