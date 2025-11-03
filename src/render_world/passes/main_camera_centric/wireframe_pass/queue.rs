use crate::prelude::*;
use crate::render_world::graphics_context::resources::{RenderDevice, RenderQueue};
use crate::render_world::passes::main_camera_centric::opaque_pass::extract::RenderTransformComponent;
use crate::render_world::passes::main_camera_centric::wireframe_pass::startup::setup_wireframe_pipeline::{
    WireframeObjectBuffer, WireframeObjectData, WireframePipeline,
};
use crate::simulation_world::chunk::consts::*;
use bevy_ecs::prelude::*;
use glam::Mat4;

/// A simple system to run when wireframes are disabled to ensure the buffer is cleared.
pub fn clear_wireframe_buffer_system(mut wireframe_buffer: ResMut<WireframeObjectBuffer>) {
    wireframe_buffer.objects.clear();
}

#[instrument(skip_all)]
pub fn queue_wireframe_system(
    mut wireframe_buffer: ResMut<WireframeObjectBuffer>,
    chunk_query: Query<&RenderTransformComponent>,
    queue: Res<RenderQueue>,
    device: Res<RenderDevice>,
    wireframe_pipeline: Res<WireframePipeline>,
) {
    wireframe_buffer.objects.clear();

    let translation_matrix = Mat4::from_translation(glam::vec3(
        (CHUNK_WIDTH - 1) as f32 / 2.0,
        (CHUNK_HEIGHT - 1) as f32 / 2.0,
        (CHUNK_DEPTH - 1) as f32 / 2.0,
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

    let buffer_size =
        (wireframe_buffer.objects.len() * std::mem::size_of::<WireframeObjectData>()) as u64;

    if wireframe_buffer.buffer.size() < buffer_size {
        let new_size = (buffer_size as f64 * 1.5).ceil() as u64;

        wireframe_buffer.buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Wireframe Object Buffer"),
            size: new_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        wireframe_buffer.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Wireframe Object Bind Group"),
            layout: &wireframe_pipeline.inner.get_layout(2),
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
