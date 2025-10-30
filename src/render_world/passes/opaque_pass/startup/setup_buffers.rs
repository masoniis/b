use crate::render_world::graphics_context::resources::RenderDevice;
use crate::render_world::passes::opaque_pass::startup::OpaquePipeline;
use crate::{prelude::*, render_world::textures::TextureArrayResource};
use bevy_ecs::prelude::*;
use bytemuck::{Pod, Zeroable};

// INFO: ------------------------
//         Custom Buffers
// ------------------------------

#[derive(Resource)]
pub struct OpaqueMaterialBindGroup(pub wgpu::BindGroup);

#[derive(Resource)]
pub struct OpaqueObjectBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub objects: Vec<OpaqueObjectData>,
}

const INITIAL_OPAQUE_OBJECT_BUFFER_CAPACITY: usize = 128;

// INFO: ---------------------------
//         Buffer data types
// ---------------------------------

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct OpaqueObjectData {
    pub model_matrix: [f32; 16],
}

// INFO: -----------------------------
//         System to set em up
// -----------------------------------

#[instrument(skip_all)]
pub fn setup_opaque_buffers_and_bind_groups(
    // Input
    device: Res<RenderDevice>,
    pipeline: Res<OpaquePipeline>,
    texture_array: Res<TextureArrayResource>,

    // Output (insert buffer resources into world)
    mut commands: Commands,
) {
    // NOTE: view buffer creation (@group(0)) is delegated to the system in `core/view`

    // NOTE: material bind group creation (@group(1))
    let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Opaque Material Bind Group"),
        layout: &pipeline.material_bind_group_layout,
        entries: &[
            // @binding(0)
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_array.array.view),
            },
            // @binding(1)
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture_array.array.sampler),
            },
        ],
    });

    commands.insert_resource(OpaqueMaterialBindGroup(material_bind_group));

    // NOTE: object buffer creation (@group(2))
    let object_buffer_size = (INITIAL_OPAQUE_OBJECT_BUFFER_CAPACITY as u64)
        * std::mem::size_of::<OpaqueObjectData>() as u64;

    let object_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Opaque Object Buffer"),
        size: object_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Opaque Object Bind Group"),
        layout: &pipeline.object_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: object_buffer.as_entire_binding(),
        }],
    });

    commands.insert_resource(OpaqueObjectBuffer {
        buffer: object_buffer,
        bind_group: object_bind_group,
        objects: Vec::with_capacity(INITIAL_OPAQUE_OBJECT_BUFFER_CAPACITY),
    });
}
