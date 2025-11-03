use crate::prelude::*;
use crate::render_world::global_extract::RenderMeshStorageResource;
use crate::render_world::passes::core::{RenderContext, RenderNode};
use crate::render_world::passes::main_camera_centric::shared::CentralCameraViewBuffer;
use crate::render_world::passes::main_camera_centric::transparent_pass::extract::TransparentRenderMeshComponent;
use crate::render_world::passes::main_camera_centric::transparent_pass::queue::Transparent3dRenderPhase;
use crate::render_world::passes::main_camera_centric::transparent_pass::startup::{
    TransparentMaterialBindGroup, TransparentObjectBuffer, TransparentPipeline,
};
use crate::render_world::passes::main_camera_centric::{
    opaque_pass::startup::DepthTextureResource,
    shared::shared_environment_buffer::EnvironmentBuffer,
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
            Some(object_buffer),
            Some(depth_texture),
            Some(pipeline),
            Some(skybox_params),
        ) = (
            world.get_resource::<Transparent3dRenderPhase>(),
            world.get_resource::<RenderMeshStorageResource>(),
            world.get_resource::<CentralCameraViewBuffer>(),
            world.get_resource::<TransparentMaterialBindGroup>(),
            world.get_resource::<TransparentObjectBuffer>(),
            world.get_resource::<DepthTextureResource>(),
            world.get_resource::<TransparentPipeline>(),
            world.get_resource::<EnvironmentBuffer>(),
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

        render_pass.set_pipeline(&pipeline.pipeline.pipeline);

        render_pass.set_bind_group(0, &view_buffer.bind_group, &[]);
        render_pass.set_bind_group(1, &skybox_params.bind_group, &[]);
        render_pass.set_bind_group(2, &material_bind_group.0, &[]);
        render_pass.set_bind_group(3, &object_buffer.bind_group, &[]);

        // INFO: --------------------------------------
        //         iterate meshes and draw them
        // --------------------------------------------
        for (i, item) in phase.items.iter().enumerate() {
            if let Ok(render_mesh_comp) = self.mesh_query.get(world, item.entity) {
                if let Some(gpu_mesh) = mesh_storage.meshes.get(&render_mesh_comp.mesh_handle.id())
                {
                    let object_index = i as u32;

                    // buffers for this specific mesh
                    render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(
                        gpu_mesh.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );

                    render_pass.draw_indexed(
                        0..gpu_mesh.index_count,
                        0,
                        object_index..(object_index + 1),
                    );
                }
            }
        }
    }
}
