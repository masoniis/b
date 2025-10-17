use crate::render_world::{
    global_extract::{
        mesh::{RenderMeshComponent, RenderTransformComponent},
        RenderMeshStorageResource,
    },
    passes::{
        main_pass::{
            prepare::{
                bind_groups::ModelBindGroup,
                resources::{MainTextureBindGroup, ViewBindGroup},
                systems::prepare_pipelines::MESH_PIPELINE_ID,
                DepthTextureResource,
            },
            queue::Opaque3dRenderPhase,
        },
        render_graph::{RenderContext, RenderNode},
    },
    resources::PipelineCacheResource,
    uniforms::ModelUniform,
};
use bevy_ecs::prelude::*;

pub struct RenderPassNode {
    // caches the queries
    mesh_query: QueryState<&'static RenderMeshComponent>,
    transform_query: QueryState<&'static RenderTransformComponent>,
}

impl RenderPassNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            mesh_query: world.query::<&RenderMeshComponent>(),
            transform_query: world.query::<&RenderTransformComponent>(),
        }
    }
}

impl RenderNode for RenderPassNode {
    fn run(&mut self, render_context: &mut RenderContext, world: &World) {
        // --- 1. Get all necessary resources from the world ---
        let phase = world.get_resource::<Opaque3dRenderPhase>().unwrap();
        let mesh_storage = world.get_resource::<RenderMeshStorageResource>().unwrap();
        let view_bind_group = world.get_resource::<ViewBindGroup>().unwrap();
        let texture_bind_group = world.get_resource::<MainTextureBindGroup>().unwrap();
        let depth_texture = world.get_resource::<DepthTextureResource>().unwrap();
        let model_bind_group = world.get_resource::<ModelBindGroup>().unwrap();
        let pipeline_cache = world.get_resource::<PipelineCacheResource>().unwrap();
        let pipeline = pipeline_cache.get(MESH_PIPELINE_ID).unwrap();

        // --- 2. Prepare and upload uniform data (same as before) ---
        let mut model_uniform_data: Vec<ModelUniform> = Vec::with_capacity(phase.items.len());
        for item in &phase.items {
            // Use the cached query: self.transform_query
            if let Ok(transform_comp) = self.transform_query.get(world, item.entity) {
                model_uniform_data.push(ModelUniform::new(
                    transform_comp.transform.to_cols_array_2d(),
                ));
            }
        }

        if !model_uniform_data.is_empty() {
            render_context.queue.write_buffer(
                &model_bind_group.buffer,
                0,
                bytemuck::cast_slice(&model_uniform_data),
            );
        }

        // --- 3. Begin the Render Pass ---
        let mut render_pass =
            render_context
                .encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Main Scene Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: render_context.surface_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(0.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

        // --- 4. The Core Render Loop (inside the render pass) ---
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &view_bind_group.bind_group, &[]);
        render_pass.set_bind_group(1, &texture_bind_group.0, &[]);

        for (i, item) in phase.items.iter().enumerate() {
            // Use the cached query: self.mesh_query
            if let Ok(render_mesh_comp) = self.mesh_query.get(world, item.entity) {
                if let Some(gpu_mesh) = mesh_storage.meshes.get(&render_mesh_comp.mesh_handle.id())
                {
                    // This is a dynamic offset into the *one* large buffer we uploaded earlier
                    let dynamic_offset = (i * std::mem::size_of::<ModelUniform>()) as u32;

                    render_pass.set_bind_group(2, &model_bind_group.bind_group, &[dynamic_offset]);
                    render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(
                        gpu_mesh.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    render_pass.draw_indexed(0..gpu_mesh.index_count, 0, 0..1);
                }
            }
        }
    }
}
