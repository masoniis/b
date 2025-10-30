use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderCameraResource,
        graphics_context::resources::{RenderDevice, RenderQueue},
        passes::core::ViewBindGroupLayout,
    },
};
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

/// A GPU buffer resource containing the shared camera view data for all camera-based passes.
#[derive(Resource)]
pub struct SharedCameraViewBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

/// The data structure representing the camera view information to be stored in the view buffer.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SharedCameraViewData {
    pub view_proj_matrix: [f32; 16],
}

/// A system to setup the shared camera view buffer and its associated bind group, @group(0).
#[instrument(skip_all)]
pub fn setup_camera_view_buffer_system(
    // Input
    device: Res<RenderDevice>,
    view_layout: Res<ViewBindGroupLayout>,

    // Output (insert buffer resources into world)
    mut commands: Commands,
) {
    let view_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Opaque View Buffer"),
        size: std::mem::size_of::<SharedCameraViewData>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Opaque View Bind Group"),
        layout: &view_layout.0,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: view_buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(SharedCameraViewBuffer {
        buffer: view_buffer,
        bind_group: view_bind_group,
    });
}

/// Takes the extracted camera data and uploads it to the GPU buffer for any pass that needs it.
///
/// Since the camera is essentially constantly rechanging this needs to be run just about
/// every frame. I am not sure it is worth adding the optimization to only run on camera
/// updates as the write buffer here is pretty cheap.
#[instrument(skip_all)]
pub fn update_camera_view_buffer_system(
    // Input
    camera_info: Res<RenderCameraResource>,
    view_buffer: Res<SharedCameraViewBuffer>,

    // Output (writing buffer to queue)
    queue: Res<RenderQueue>,
) {
    let view_proj_matrix = camera_info.projection_matrix * camera_info.view_matrix;

    let camera_data = SharedCameraViewData {
        view_proj_matrix: view_proj_matrix.to_cols_array(),
    };

    queue.write_buffer(&view_buffer.buffer, 0, bytemuck::cast_slice(&[camera_data]));
}
