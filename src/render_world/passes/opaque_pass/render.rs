use crate::prelude::*;
use crate::render_world::passes::core::{RenderContext, RenderNode};
use crate::render_world::passes::opaque_pass::queue::Opaque3dRenderPhase;
use crate::render_world::passes::opaque_pass::startup::{
    DepthTextureResource, OpaqueMaterialBindGroup, OpaqueObjectBuffer, OpaqueObjectData,
    OpaqueViewBuffer,
};
use crate::render_world::{
    global_extract::{
        mesh::{RenderMeshComponent, RenderTransformComponent},
        RenderMeshStorageResource,
    },
    passes::opaque_pass::startup::OpaquePipeline,
};
use bevy_ecs::prelude::*;

pub struct OpaquePassRenderNode {
    // caches the queries
    mesh_query: QueryState<&'static RenderMeshComponent>,
    transform_query: QueryState<&'static RenderTransformComponent>,
}

impl OpaquePassRenderNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            mesh_query: world.query::<&RenderMeshComponent>(),
            transform_query: world.query::<&RenderTransformComponent>(),
        }
    }
}

impl RenderNode for OpaquePassRenderNode {
    #[instrument(skip_all, name = "opaque_pass_render_node")]
    fn run(&mut self, render_context: &mut RenderContext, world: &World) {
        // --- 1. Get all necessary resources from the world ---
        let phase = world.get_resource::<Opaque3dRenderPhase>().unwrap();
        let mesh_storage = world.get_resource::<RenderMeshStorageResource>().unwrap();
        let view_buffer = world.get_resource::<OpaqueViewBuffer>().unwrap();
        let material_bind_group = world.get_resource::<OpaqueMaterialBindGroup>().unwrap();
        let object_buffer = world.get_resource::<OpaqueObjectBuffer>().unwrap();
        let depth_texture = world.get_resource::<DepthTextureResource>().unwrap();
        let opaque_pipeline = world.get_resource::<OpaquePipeline>().unwrap();
        let pipeline = &opaque_pipeline.pipeline; // Get the pipeline from our resource

        // --- 2. Prepare and upload object data (to the SSBO) ---
        //    (We check capacity and resize the buffer if needed first)
        let mut object_data: Vec<OpaqueObjectData> = Vec::with_capacity(phase.items.len());
        for item in &phase.items {
            // Use the cached query: self.transform_query
            if let Ok(transform_comp) = self.transform_query.get(world, item.entity) {
                object_data.push(OpaqueObjectData {
                    model_matrix: transform_comp.transform.to_cols_array(),
                });
            }
        }

        if !object_data.is_empty() {
            // TODO: Add buffer resizing logic here if object_data.len() > capacity
            render_context.queue.write_buffer(
                &object_buffer.buffer,
                0,
                bytemuck::cast_slice(&object_data),
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
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
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

        // Bind groups 0, 1, and 2 *ONCE* outside the loop
        render_pass.set_bind_group(0, &view_buffer.bind_group, &[]);
        render_pass.set_bind_group(1, &material_bind_group.0, &[]);
        render_pass.set_bind_group(2, &object_buffer.bind_group, &[]);

        for (i, item) in phase.items.iter().enumerate() {
            // Use the cached query: self.mesh_query
            if let Ok(render_mesh_comp) = self.mesh_query.get(world, item.entity) {
                if let Some(gpu_mesh) = mesh_storage.meshes.get(&render_mesh_comp.mesh_handle.id())
                {
                    // This is the index into the SSBO
                    let object_index = i as u32;

                    // Set buffers for this specific mesh
                    render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(
                        gpu_mesh.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );

                    // Draw the mesh, passing the `object_index`
                    // to the shader via `instance_index`.
                    render_pass.draw_indexed(
                        0..gpu_mesh.index_count,
                        0,
                        object_index..(object_index + 1), // This sends the index!
                    );
                }
            }
        }
    }
}
