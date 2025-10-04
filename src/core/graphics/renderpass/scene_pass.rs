use crate::core::graphics::rendercore::setup::MAX_TRANSFORMS;
use crate::core::graphics::renderpass::traits::ISceneRenderPass;
use crate::render_world::extract::resources::RenderMeshStorageResource;
use crate::render_world::queue::RenderQueueResource;
use crate::{
    core::graphics::renderpass::RenderPassContex, core::graphics::types::instance::InstanceRaw,
    game_world::global_resources::asset_storage::AssetId,
};
use std::{collections::HashMap, sync::Arc};

pub struct SceneRenderPass {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}

impl SceneRenderPass {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self { device, queue }
    }
}

impl ISceneRenderPass for SceneRenderPass {
    fn prepare(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _render_queue: &RenderQueueResource,
        _render_mesh_storage: &RenderMeshStorageResource,
        _camera_info: &crate::render_world::extract::RenderCameraResource,
    ) {
        // No actual preparation needed here for SceneRenderPass, as it's done in render
    }

    fn render<'a>(
        &'a self,
        encoder: &mut wgpu::CommandEncoder,
        context: RenderPassContex<'a>,
        ecs_render_queue: &RenderQueueResource,
        render_mesh_storage: &RenderMeshStorageResource,
        instance_buffer: &wgpu::Buffer,
        render_pipeline: &wgpu::RenderPipeline,
        texture_bind_group: &wgpu::BindGroup,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Scene Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: context.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // Clear the screen with the background color
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0075,
                        g: 0.0125,
                        b: 0.0250,
                        a: 1.0000,
                    }),
                    // Store the result to be used by the next pass
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            // Use the depth buffer
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: context.depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0), // Clear depth to 1.0
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let mut instances: Vec<InstanceRaw> = Vec::new();
        let mut instance_offsets: HashMap<AssetId, (u32, u32)> = HashMap::new(); // mesh_handle.id() -> (offset, count)

        let mut total_instances_written = 0;

        for (mesh_handle, draws) in ecs_render_queue.iter_by_mesh() {
            let start_offset = total_instances_written;
            let instances_for_mesh = draws
                .len()
                .min(MAX_TRANSFORMS as usize - total_instances_written);

            for draw in draws.iter().take(instances_for_mesh) {
                instances.push(InstanceRaw {
                    model_matrix: draw.transform.to_cols_array_2d(),
                });
            }
            instance_offsets.insert(
                mesh_handle.id(),
                (start_offset as u32, instances_for_mesh as u32),
            );
            total_instances_written += instances_for_mesh;
        }

        if total_instances_written > MAX_TRANSFORMS as usize {
            tracing::warn!(
                "Number of queued draws ({}) exceeds MAX_TRANSFORMS ({}). Only rendering the first {} transforms.",
                total_instances_written, MAX_TRANSFORMS, MAX_TRANSFORMS
            );
        }

        self.queue
            .write_buffer(instance_buffer, 0, bytemuck::cast_slice(&instances));

        render_pass.set_pipeline(render_pipeline);
        render_pass.set_bind_group(0, &context.shared_data.camera_bind_group, &[]);
        render_pass.set_bind_group(1, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

        for (mesh_handle, _draws) in ecs_render_queue.iter_by_mesh() {
            // This is the second loop, where draws is unused
            let gpu_mesh = render_mesh_storage
                .meshes
                .get(&mesh_handle.id())
                .unwrap_or_else(|| {
                    panic!(
                        "GpuMesh not found in RenderMeshStorageResource for handle ID: {}",
                        mesh_handle.id()
                    )
                });

            render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(gpu_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            if let Some(&(offset, count)) = instance_offsets.get(&mesh_handle.id()) {
                render_pass.draw_indexed(0..gpu_mesh.index_count, 0, offset..(offset + count));
            } else {
                tracing::warn!(
                    "Mesh handle {:?} not found in instance_offsets.",
                    mesh_handle.id()
                );
            }
        }
    }
}
