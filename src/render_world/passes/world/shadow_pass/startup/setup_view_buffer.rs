use crate::prelude::*;
use crate::render_world::graphics_context::resources::RenderDevice;
use crate::render_world::passes::world::shadow_pass::startup::ShadowPassPipeline;
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

// INFO: -----------------------------------
//         GPU buffer and data types
// -----------------------------------------

/// A GPU buffer resource containing the shadow pass's view data.
#[derive(Resource)]
pub struct ShadowViewBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

/// The data structure representing the shadow "camera" (i.e., the sun's) view.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct ShadowViewData {
    pub light_view_proj_matrix: [f32; 16],
}

// INFO: ----------------------------
//          management systems
// ----------------------------------

/// A system to setup the shadow view buffer and its associated bind group.
#[instrument(skip_all)]
pub fn setup_shadow_view_buffer_system(
    // Input
    device: Res<RenderDevice>,
    pipeline: Res<ShadowPassPipeline>,

    // Output (insert buffer resources into world)
    mut commands: Commands,
) {
    let view_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Shadow View Buffer"),
        size: std::mem::size_of::<ShadowViewData>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Shadow View Bind Group"),
        layout: &pipeline.get_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: view_buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(ShadowViewBuffer {
        buffer: view_buffer,
        bind_group: view_bind_group,
    });
}
