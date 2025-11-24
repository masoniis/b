use crate::prelude::*;
use crate::render_world::passes::world::gpu_resources::world_uniforms::ChunkStorageManager;
use crate::render_world::passes::world::main_passes::shared_resources::TextureArrayUniforms;
use crate::render_world::{
    global_extract::RenderMeshStorageResource,
    passes::core::{RenderContext, RenderNode},
    passes::world::main_passes::{
        shared_resources::{
            main_depth_texture::MainDepthTextureResource, CentralCameraViewUniform,
            EnvironmentUniforms,
        },
        transparent_pass::{
            extract::TransparentRenderMeshComponent, queue::Transparent3dRenderPhase,
            startup::TransparentPipeline,
        },
    },
};
use bevy_ecs::prelude::*;

pub struct TransparentPassRenderNode {
    // caches the queries
    mesh_query: QueryState<&'static TransparentRenderMeshComponent>,
}

impl TransparentPassRenderNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            mesh_query: world.query::<&TransparentRenderMeshComponent>(),
        }
    }
}

impl RenderNode for TransparentPassRenderNode {
    #[instrument(skip_all, name = "transparent_pass_render_node")]
    fn run(&mut self, render_context: &mut RenderContext, world: &World) {
        // INFO: -------------------------------------
        //         collect rendering resources
        // -------------------------------------------
        let (
            Some(phase),
            Some(mesh_storage),
            Some(view_buffer),
            Some(material_bind_group),
            Some(depth_texture),
            Some(pipeline),
            Some(skybox_params),
            Some(chunk_memory_manager),
        ) = (
            world.get_resource::<Transparent3dRenderPhase>(),
            world.get_resource::<RenderMeshStorageResource>(),
            world.get_resource::<CentralCameraViewUniform>(),
            world.get_resource::<TextureArrayUniforms>(),
            world.get_resource::<MainDepthTextureResource>(),
            world.get_resource::<TransparentPipeline>(),
            world.get_resource::<EnvironmentUniforms>(),
            world.get_resource::<ChunkStorageManager>(),
        )
        else {
            warn!(
                "Missing one or more required resources for the Transparent Pass. Skipping pass."
            );
            return;
        };

        // INFO: --------------------------------
        //         set up the render pass
        // --------------------------------------
        let mut render_pass =
            render_context
                .encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Transparent Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: render_context.surface_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load, // Load the existing frame
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load, // Load the depth buffer
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

        // INFO: -----------------------------------------
        //         mesh pipeline: iterate and draw
        // -----------------------------------------------
        render_pass.set_pipeline(&pipeline.pipeline);

        render_pass.set_bind_group(0, &view_buffer.bind_group, &[]);
        render_pass.set_bind_group(1, &skybox_params.bind_group, &[]);
        render_pass.set_bind_group(2, &material_bind_group.bind_group, &[]);
        render_pass.set_bind_group(3, &chunk_memory_manager.bind_group, &[]);

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
