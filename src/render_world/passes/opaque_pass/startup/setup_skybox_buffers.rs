use crate::prelude::*;
use crate::render_world::graphics_context::resources::RenderDevice;
use crate::render_world::passes::opaque_pass::startup::OpaquePipelines;
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

// INFO: ------------------------
//         Custom Buffers
// ------------------------------

#[derive(Resource)]
pub struct SkyboxParamsBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

// INFO: ---------------------------
//         Buffer data types
// ---------------------------------

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct SkyboxParamsData {
    pub sun_direction: [f32; 3],
    _padding1: u32,
    pub horizon_color: [f32; 3],
    _padding2: u32,
    pub zenith_color: [f32; 3],
    _padding3: u32,
}

impl SkyboxParamsData {
    pub fn new(sun_direction: [f32; 3], horizon_color: [f32; 3], zenith_color: [f32; 3]) -> Self {
        Self {
            sun_direction,
            _padding1: 0,
            horizon_color,
            _padding2: 0,
            zenith_color,
            _padding3: 0,
        }
    }
}

// INFO: -----------------------------
//         System to set em up
// -----------------------------------

#[instrument(skip_all)]
pub fn setup_skybox_params_buffer_system(
    // Input
    device: Res<RenderDevice>,
    pipelines: Res<OpaquePipelines>,

    // Output (insert buffer resource into world)
    mut commands: Commands,
) {
    // NOTE: view buffer creation (@group(0)) is handled by the shared `core/view` system

    // NOTE: skybox params buffer creation (@group(1))
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Skybox Params Buffer"),
        size: std::mem::size_of::<SkyboxParamsData>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Skybox Params Bind Group"),
        layout: &pipelines.skybox.pipeline.get_bind_group_layout(1),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(SkyboxParamsBuffer { buffer, bind_group });
}
