use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderQueue};
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

// INFO: -----------------------------------
//         GPU Buffer and Data types
// -----------------------------------------

/// The environment bind group layout resource shared by all camera-centric render passes.
#[derive(Resource)]
pub struct EnvironmentBindGroupLayout(pub wgpu::BindGroupLayout);

/// A GPU buffer resource containing the shared environment data for all central-camera-based passes.
#[derive(Resource)]
pub struct EnvironmentBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct EnvironmentData {
    pub sun_direction: [f32; 3],
    _padding1: u32,
    pub horizon_color: [f32; 3],
    _padding2: u32,
    pub zenith_color: [f32; 3],
    _padding3: u32,
}

impl EnvironmentData {
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

/// A system to setup the environment bind group for camera-centric passes
#[instrument(skip_all)]
pub fn setup_environment_layout_system(mut commands: Commands, device: Res<RenderDevice>) {
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Environment Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    commands.insert_resource(EnvironmentBindGroupLayout(layout));
}

/// Sets up the environment buffer and its bind group (@group(1)) for the camera-centric passes
#[instrument(skip_all)]
pub fn setup_environment_buffer_system(
    // Input
    device: Res<RenderDevice>,
    layout: Res<EnvironmentBindGroupLayout>,

    // Output (insert buffer resource into world)
    mut commands: Commands,
) {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Environment Buffer"),
        size: std::mem::size_of::<EnvironmentData>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Environment Bind Group"),
        layout: &layout.0,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(EnvironmentBuffer { buffer, bind_group });
}

/// A system to prepare the environment buffer data for centric passes.
#[instrument(skip_all)]
pub fn prepare_environment_buffer_system(
    // Input (target buffer)
    buffer: Res<EnvironmentBuffer>,

    // Output (writing buffer to queue)
    queue: Res<RenderQueue>,
) {
    let skybox_params = EnvironmentData::new([0.0, 0.0, 0.0], [0.08, 0.12, 0.45], [0.0, 0.0, 0.0]);

    queue.write_buffer(&buffer.buffer, 0, bytemuck::cast_slice(&[skybox_params]));
}
