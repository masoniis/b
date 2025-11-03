use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderCameraResource,
        graphics_context::resources::{RenderDevice, RenderQueue},
        passes::main_camera_centric::{
            opaque_pass::extract::RenderTransformComponent,
            transparent_pass::{
                extract::TransparentRenderMeshComponent,
                startup::{TransparentObjectBuffer, TransparentObjectData, TransparentPipeline},
            },
        },
    },
};
use bevy_ecs::prelude::*;

#[derive(Debug)]
pub struct PhaseItem {
    pub entity: Entity,
    pub distance: f32, // for sorting back-to-front
}

#[derive(Resource, Default)]
pub struct Transparent3dRenderPhase {
    pub items: Vec<PhaseItem>,
}

// A temporary struct to hold all the data we need for sorting
struct SortableTransparentItem {
    distance: f32,
    entity: Entity,
    model_matrix: [f32; 16],
}

/// The system responsible for populating the `RenderQueueResource`.
///
/// Performs a query for all entities that have been extracted into the render
/// world and adds them to a list of draw calls for the renderer to consume.
#[instrument(skip_all)]
pub fn queue_and_prepare_transparent_system(
    // Input
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    pipeline: Res<TransparentPipeline>,
    camera_info: Res<RenderCameraResource>,
    meshes_query: Query<(
        Entity,
        &TransparentRenderMeshComponent,
        &RenderTransformComponent,
    )>,

    // Output
    mut transparent_phase: ResMut<Transparent3dRenderPhase>,
    mut object_buffer: ResMut<TransparentObjectBuffer>,
) {
    transparent_phase.items.clear();
    object_buffer.objects.clear();

    // collect sortable items for the render pass
    let camera_position = camera_info.world_position;
    let mut sortable_items: Vec<SortableTransparentItem> =
        Vec::with_capacity(meshes_query.iter().len());
    for (entity, _mesh, transform) in meshes_query.iter() {
        // TODO: Frustum culling here

        let object_position = transform.transform.w_axis.truncate();
        let distance_from_camera = (object_position - camera_position).length_squared();

        sortable_items.push(SortableTransparentItem {
            distance: distance_from_camera,
            entity,
            model_matrix: transform.transform.to_cols_array(),
        });
    }

    // sort by back to front for transparency
    sortable_items.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    // populate the phase and object buffer in correct sorted order
    for item in sortable_items {
        transparent_phase.items.push(PhaseItem {
            entity: item.entity,
            distance: item.distance,
        });
        object_buffer.objects.push(TransparentObjectData {
            model_matrix: item.model_matrix,
        });
    }

    if object_buffer.objects.is_empty() {
        return; // no objects, nothing more to do
    }

    // check if the GPU buffer needs to be resized to fit all objects
    let required_size_bytes = (object_buffer.objects.len()
        * std::mem::size_of::<TransparentObjectData>())
        as wgpu::BufferAddress;

    if required_size_bytes > object_buffer.buffer.size() {
        info!(
            "Resizing transparent object buffer to fit data (current size = {} bytes, required size = {} bytes)",
            object_buffer.buffer.size(),
            required_size_bytes
        );

        // calculate new amortized size and updated the buffer and bind group
        let new_capacity_elements = (object_buffer.objects.len() as f64 * 1.5).ceil() as usize;
        let new_size_bytes = (new_capacity_elements * std::mem::size_of::<TransparentObjectData>())
            as wgpu::BufferAddress;

        let new_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transparent Object Buffer"),
            size: new_size_bytes,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let new_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transparent Object Bind Group"),
            layout: &pipeline.pipeline.get_layout(3),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: new_buffer.as_entire_binding(),
            }],
        });

        object_buffer.buffer = new_buffer;
        object_buffer.bind_group = new_bind_group;
    }

    // write data to the buffer (which might be new/resized)
    queue.write_buffer(
        &object_buffer.buffer,
        0,
        bytemuck::cast_slice(&object_buffer.objects),
    );
}
