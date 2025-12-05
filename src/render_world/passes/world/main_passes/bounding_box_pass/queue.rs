use crate::render_world::passes::world::main_passes::bounding_box_pass::gpu_resources::object_binding::WireframeObjectBindGroupLayout;
use crate::prelude::*;
use crate::render_world::passes::world::main_passes::bounding_box_pass::extract::WireframeToggleState;
use crate::render_world::{
    graphics_context::resources::{RenderDevice, RenderQueue},
    passes::world::main_passes::{
        bounding_box_pass::gpu_resources::{
            WireframeObjectBuffer, WireframeObjectData,
        },
        opaque_pass::extract::RenderTransformComponent,
    },
};
use crate::simulation_world::block::TargetedBlock;
use crate::simulation_world::chunk::consts::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use bevy_ecs::prelude::*;

#[instrument(skip_all)]
pub fn queue_wireframe_system(
    // input
    queue: Res<RenderQueue>,
    device: Res<RenderDevice>,

    object_layout: Res<WireframeObjectBindGroupLayout>,

    chunk_query: Query<&RenderTransformComponent>,
    active_bounds: Res<WireframeToggleState>,
    targeted_block: Res<TargetedBlock>,

    // output
    mut wireframe_buffer: ResMut<WireframeObjectBuffer>,
) {
    wireframe_buffer.objects.clear();

    if !active_bounds.enabled && targeted_block.position.is_none() {
        return;
    }

    if active_bounds.enabled {
        // add all chunk wireframes
        let translation_matrix = Mat4::from_translation(glam::vec3(
            CHUNK_WIDTH as f32 / 2.0,
            CHUNK_HEIGHT as f32 / 2.0,
            CHUNK_DEPTH as f32 / 2.0,
        ));
        let scale_matrix = Mat4::from_scale(glam::vec3(
            CHUNK_WIDTH as f32,
            CHUNK_HEIGHT as f32,
            CHUNK_DEPTH as f32,
        ));

        for transform in chunk_query.iter() {
            let model_matrix = transform.transform * translation_matrix * scale_matrix;

            wireframe_buffer.objects.push(WireframeObjectData {
                model_matrix: model_matrix.to_cols_array(),
            });
        }
    }

    // add targeted block wireframe if exists
    if let Some(block_pos) = targeted_block.position {
        let block_translation = glam::vec3(
            block_pos.x as f32 + 0.5,
            block_pos.y as f32 + 0.5,
            block_pos.z as f32 + 0.5,
        );
        let block_translation_matrix = Mat4::from_translation(block_translation);
        let model_matrix = Mat4::IDENTITY * block_translation_matrix;

        wireframe_buffer.objects.push(WireframeObjectData {
            model_matrix: model_matrix.to_cols_array(),
        });
    }

    if !wireframe_buffer.objects.is_empty() {
        let buffer_size =
            (wireframe_buffer.objects.len() * std::mem::size_of::<WireframeObjectData>()) as u64;

        if wireframe_buffer.buffer.size() < buffer_size {
            let new_size = (buffer_size as f64 * 1.5).ceil() as u64;

            // destroy old buffer, it is being replaced
            wireframe_buffer.buffer.destroy();

            wireframe_buffer.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Wireframe Object Buffer"),
                size: new_size,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            wireframe_buffer.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Wireframe Object Bind Group"),
                layout: &object_layout.0,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wireframe_buffer.buffer.as_entire_binding(),
                }],
            });
        }

        // write data to the buffer (which might be new/resized)
        queue.write_buffer(
            &wireframe_buffer.buffer,
            0,
            bytemuck::cast_slice(&wireframe_buffer.objects),
        );
    }
}
