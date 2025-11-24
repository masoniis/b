use crate::prelude::*;
use crate::render_world::passes::world::gpu_resources::world_uniforms::ChunkStorageManager;
use crate::render_world::{
    global_extract::RenderMeshStorageResource,
    passes::core::{RenderContext, RenderNode},
    passes::world::main_passes::opaque_pass::{
        extract::OpaqueRenderMeshComponent, queue::Opaque3dRenderPhase,
    },
    passes::world::shadow_pass::gpu_resources::{
        ShadowDepthTextureResource, ShadowPassPipeline, ShadowViewBuffer,
    },
};
use bevy_ecs::prelude::*;

pub struct ShadowRenderPassNode {
    // caches the queries
    mesh_query: QueryState<&'static OpaqueRenderMeshComponent>,
}

impl ShadowRenderPassNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            mesh_query: world.query::<&OpaqueRenderMeshComponent>(),
        }
    }
}

impl RenderNode for ShadowRenderPassNode {
    #[instrument(skip_all, name = "shadow_pass_render_node")]
    fn run(&mut self, render_context: &mut RenderContext, world: &World) {
        // INFO: -------------------------------------
        //          collect rendering resources
        // -------------------------------------------
        let (
            // shadow-specific stuff
            Some(pipeline),
            Some(shadow_view_buffer),
            Some(shadow_depth_texture),
            // opaque mesh to base shadow depth on
            Some(phase),
            Some(mesh_storage),
            Some(chunk_memory_manager),
        ) = (
            world.get_resource::<ShadowPassPipeline>(),
            world.get_resource::<ShadowViewBuffer>(),
            world.get_resource::<ShadowDepthTextureResource>(),
            world.get_resource::<Opaque3dRenderPhase>(),
            world.get_resource::<RenderMeshStorageResource>(),
            world.get_resource::<ChunkStorageManager>(),
        )
        else {
            warn!("Missing one or more required resources for the Shadow Pass. Skipping pass.");
            return;
        };

        // INFO: --------------------------------
        //         set up the render pass
        // --------------------------------------
        let mut render_pass =
            render_context
                .encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Shadow Map Render Pass"),
                    color_attachments: &[],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &shadow_depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

        // INFO: -------------------------------------------
        //         shadow pipeline: iterate and draw
        // -------------------------------------------------
        render_pass.set_pipeline(&pipeline.pipeline);

        render_pass.set_bind_group(0, &shadow_view_buffer.bind_group, &[]);
        render_pass.set_bind_group(1, &chunk_memory_manager.bind_group, &[]);

        for item in phase.items.iter() {
            if let Ok(render_mesh_comp) = self.mesh_query.get(world, item.entity) {
                if let Some(gpu_mesh) = mesh_storage.meshes.get(&render_mesh_comp.mesh_handle.id())
                {
                    let object_index = gpu_mesh.slot_index;

                    render_pass.draw(
                        0..(gpu_mesh.face_count * 6),
                        object_index..(object_index + 1),
                    );
                }
            }
        }
    }
}
