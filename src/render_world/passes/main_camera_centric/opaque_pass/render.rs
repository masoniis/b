use crate::prelude::*;
use crate::render_world::global_extract::RenderMeshStorageResource;
use crate::render_world::passes::core::{RenderContext, RenderNode};
use crate::render_world::passes::main_camera_centric::opaque_pass::extract::OpaqueRenderMeshComponent;
use crate::render_world::passes::main_camera_centric::opaque_pass::queue::Opaque3dRenderPhase;
use crate::render_world::passes::main_camera_centric::shared::CentralCameraViewBuffer;
use crate::render_world::passes::main_camera_centric::{
    opaque_pass::startup::{
        DepthTextureResource, OpaqueMaterialBindGroup, OpaqueObjectBuffer, OpaquePipelines,
        OpaqueRenderMode,
    },
    shared::shared_environment_buffer::EnvironmentBuffer,
};
use bevy_ecs::prelude::*;

pub struct OpaquePassRenderNode {
    // caches the queries
    mesh_query: QueryState<&'static OpaqueRenderMeshComponent>,
}

impl OpaquePassRenderNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            mesh_query: world.query::<&OpaqueRenderMeshComponent>(),
        }
    }
}

impl RenderNode for OpaquePassRenderNode {
    #[instrument(skip_all, name = "opaque_pass_render_node")]
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
            Some(pipelines),
            Some(render_mode),
            Some(skybox_params),
        ) = (
            world.get_resource::<Opaque3dRenderPhase>(),
            world.get_resource::<RenderMeshStorageResource>(),
            world.get_resource::<CentralCameraViewBuffer>(),
            world.get_resource::<OpaqueMaterialBindGroup>(),
            world.get_resource::<OpaqueObjectBuffer>(),
            world.get_resource::<DepthTextureResource>(),
            world.get_resource::<OpaquePipelines>(),
            world.get_resource::<OpaqueRenderMode>(),
            world.get_resource::<EnvironmentBuffer>(),
        )
        else {
            warn!("Missing one or more required resources for the Opaque Pass. Skipping pass.");
            return;
        };

        let active_pipeline = match *render_mode {
            OpaqueRenderMode::Fill => &pipelines.fill.pipeline,
            OpaqueRenderMode::Wireframe => &pipelines.wireframe.pipeline,
        };

        // INFO: --------------------------------
        //         set up the render pass
        // --------------------------------------
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

        // INFO: -------------------------
        //         skybox pipeline
        // -------------------------------
        render_pass.set_pipeline(&pipelines.skybox.pipeline);
        render_pass.set_bind_group(0, &view_buffer.bind_group, &[]);
        render_pass.set_bind_group(1, &skybox_params.bind_group, &[]);

        render_pass.draw(0..6, 0..1);

        // INFO: -----------------------------------------
        //         mesh pipeline: iterate and draw
        // -----------------------------------------------
        render_pass.set_pipeline(&active_pipeline);

        render_pass.set_bind_group(0, &view_buffer.bind_group, &[]);
        render_pass.set_bind_group(1, &skybox_params.bind_group, &[]);
        render_pass.set_bind_group(2, &material_bind_group.0, &[]);
        render_pass.set_bind_group(3, &object_buffer.bind_group, &[]);

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
