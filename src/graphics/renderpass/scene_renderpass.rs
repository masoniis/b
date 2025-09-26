use crate::graphics::rendercore::types::{ISceneRenderPass, InstanceRaw, MAX_TRANSFORMS};
use crate::{
    ecs::resources::{
        asset_storage::{AssetStorageResource, MeshAsset},
        CameraUniformResource, RenderQueueResource,
    },
    graphics::{GpuMesh, RenderContext, Vertex},
};
use std::{collections::HashMap, sync::Arc};
use wgpu::util::DeviceExt;

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
        _mesh_assets: &AssetStorageResource<MeshAsset>,
        _camera_uniform: &CameraUniformResource,
    ) {
        // No actual preparation needed here for SceneRenderPass, as it's done in render
    }

    fn render<'a>(
        &'a self,
        encoder: &mut wgpu::CommandEncoder,
        context: RenderContext<'a>,
        ecs_render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        instance_buffer: &wgpu::Buffer,
        gpu_meshes: &mut HashMap<crate::ecs::resources::asset_storage::AssetId, Arc<GpuMesh>>,
        render_pipeline: &wgpu::RenderPipeline,
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

        // --- Instance Buffer Preparation (same as before) ---
        let num_queued_draws = ecs_render_queue.get_scene_objects().len();
        if num_queued_draws > MAX_TRANSFORMS as usize {
            tracing::warn!(
                "Number of queued draws ({}) exceeds MAX_TRANSFORMS ({}). Only rendering the first {} transforms.",
                num_queued_draws, MAX_TRANSFORMS, MAX_TRANSFORMS
            );
        }

        let mut instances = Vec::with_capacity(num_queued_draws.min(MAX_TRANSFORMS as usize));
        for draw in ecs_render_queue
            .get_scene_objects()
            .iter()
            .take(MAX_TRANSFORMS as usize)
        {
            instances.push(InstanceRaw {
                model_matrix: draw.transform.to_cols_array_2d(),
            });
        }

        self.queue
            .write_buffer(instance_buffer, 0, bytemuck::cast_slice(&instances));

        render_pass.set_pipeline(render_pipeline);
        render_pass.set_bind_group(0, &context.shared_data.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

        let mut current_offset = 0;
        for (mesh_handle, draws) in ecs_render_queue.iter_by_mesh() {
            let gpu_mesh = gpu_meshes.entry(mesh_handle.id()).or_insert_with(|| {
                let mesh_asset = mesh_assets.get(mesh_handle).unwrap();
                create_gpu_mesh_from_data(&self.device, &mesh_asset.vertices, &mesh_asset.indices)
            });

            render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(gpu_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            let instances_to_draw = draws.len() as u32;
            render_pass.draw_indexed(
                0..gpu_mesh.index_count,
                0,
                current_offset..current_offset + instances_to_draw,
            );
            current_offset += instances_to_draw;
        }
    }
}

pub fn create_gpu_mesh_from_data(
    device: &wgpu::Device,
    vertices: &[Vertex],
    indices: &[u32],
) -> Arc<GpuMesh> {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    // Wrap the GpuMesh in an Arc
    Arc::new(GpuMesh {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
    })
}
