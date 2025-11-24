use crate::prelude::*;
use crate::render_world::global_extract::{ExtractedSun, RenderTimeResource};
use crate::render_world::graphics_context::resources::{RenderDevice, RenderQueue};
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

// INFO: ----------------------------
//         uniform definition
// ----------------------------------

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default)]
pub struct EnvironmentData {
    pub sun_direction: [f32; 3],
    pub time: f32, // no padding since time fills slot
    pub horizon_color: [f32; 3],
    _padding2: u32,
    pub zenith_color: [f32; 3],
    _padding3: u32,
}

impl EnvironmentData {
    pub fn new(
        sun_direction: [f32; 3],
        world_time: f32,
        horizon_color: [f32; 3],
        zenith_color: [f32; 3],
    ) -> Self {
        Self {
            sun_direction,
            time: world_time,
            horizon_color,
            _padding2: 0,
            zenith_color,
            _padding3: 0,
        }
    }
}

// INFO: -----------------------------------------
//         GPU binding, buffer, and layout
// -----------------------------------------------

/// The environment bind group layout resource shared by all camera-centric render passes.
#[derive(Resource)]
pub struct EnvironmentBindGroupLayout(pub wgpu::BindGroupLayout);

impl FromWorld for EnvironmentBindGroupLayout {
    #[instrument(skip_all)]
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Environment Bind Group Layout"),
            entries: &[
                // holds buffer for `EnvironmentData`
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        Self(layout)
    }
}

/// A GPU buffer resource containing the shared environment data for all central-camera-based passes.
#[derive(Resource)]
pub struct EnvironmentUniforms {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl FromWorld for EnvironmentUniforms {
    #[instrument(skip_all)]
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let layout = world.resource::<EnvironmentBindGroupLayout>();

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

        Self { buffer, bind_group }
    }
}

// INFO: ------------------------
//         update systems
// ------------------------------

/// A system to prepare the environment buffer data for centric passes.
#[instrument(skip_all)]
pub fn update_environment_uniform_buffer_system(
    // Input (target buffer)
    buffer: Res<EnvironmentUniforms>,
    extracted_sun: Res<ExtractedSun>,
    extracted_time: Res<RenderTimeResource>,

    // Output (writing buffer to queue)
    queue: Res<RenderQueue>,
) {
    let environtment_data = EnvironmentData::new(
        extracted_sun.direction,
        extracted_time.total_elapsed_seconds,
        [0.08, 0.12, 0.45],
        [0.3, 0.5, 0.8],
    );

    queue.write_buffer(
        &buffer.buffer,
        0,
        bytemuck::cast_slice(&[environtment_data]),
    );
}
