use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderWindowSizeResource,
        passes::ui_pass::startup::ViewBindGroupLayout, resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;
use wgpu::util::DeviceExt;

/// A resource holding the shared projection matrix for the UI.
#[derive(Resource)]
pub struct UiViewBindGroup {
    pub bind_group: wgpu::BindGroup,
}

/// A system that creates the orthographic projection matrix for the UI camera.
///
/// Run condition: If the window size or the view bind group layout changes.
#[instrument(skip_all)]
pub fn prepare_ui_view_system(
    // Input
    gfx: Res<GraphicsContextResource>,
    window_size: Res<RenderWindowSizeResource>,
    view_layout: Res<ViewBindGroupLayout>,

    // Output (insert resource)
    mut commands: Commands,
) {
    debug!(
        target : "ui_efficiency",
        "Updating UI view (this should only happen the screen was resized)..."
    );

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
        layout: &view_layout.0,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: projection_buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(UiViewBindGroup { bind_group });
}
