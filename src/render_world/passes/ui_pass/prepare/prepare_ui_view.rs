use crate::{
    prelude::*,
    render_world::{extract::RenderWindowSizeResource, resources::GraphicsContextResource},
};
use bevy_ecs::prelude::*;
use wgpu::util::DeviceExt;

use super::{UiPipeline, UiViewBindGroup};

// A system that creates the orthographic projection matrix for the UI camera.
pub fn prepare_ui_view_system(
    // Input
    gfx: Res<GraphicsContextResource>,
    pipeline: Res<UiPipeline>,
    window_size: Res<RenderWindowSizeResource>,

    // Output (insert resource)
    mut commands: Commands,
) {
    let device = &gfx.context.device;

    let projection_matrix =
        Mat4::orthographic_rh(0.0, window_size.width, window_size.height, 0.0, -1.0, 1.0);
    let projection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("UI Projection Uniform Buffer"),
        contents: bytemuck::cast_slice(projection_matrix.as_ref()),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("UI View Bind Group"),
        layout: &pipeline.view_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: projection_buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(UiViewBindGroup { bind_group });
}
