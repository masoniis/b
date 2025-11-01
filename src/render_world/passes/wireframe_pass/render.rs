use crate::{
    prelude::*,
    render_world::passes::{
            core::{view::SharedCameraViewBuffer, RenderContext, RenderNode},
            opaque_pass::startup::DepthTextureResource,
            wireframe_pass::startup::{
                setup_wireframe_mesh::DebugWireframeMesh, setup_wireframe_pipeline::*,
            },
        },
};
use bevy_ecs::prelude::*;

pub struct WireframeRenderNode;

impl RenderNode for WireframeRenderNode {
    #[instrument(skip_all, name = "wireframe_pass_render_node")]
    fn run(&mut self, render_context: &mut RenderContext, world: &World) {
        // INFO: ---------------------------
        //         resource fetching
        // ---------------------------------

        let (
            Some(wireframe_pipeline),
            Some(wireframe_buffer),
            Some(wireframe_mesh),
            Some(view_bind_group),
            Some(depth_texture),
        ) = (
            world.get_resource::<WireframePipeline>(),
            world.get_resource::<WireframeObjectBuffer>(),
            world.get_resource::<DebugWireframeMesh>(),
            world.get_resource::<SharedCameraViewBuffer>(),
            world.get_resource::<DepthTextureResource>(),
        )
        else {
            warn!("Missing one or more required resources for the Wireframe Pass. Skipping pass.");
            return;
        };

        if wireframe_buffer.objects.is_empty() {
            return;
        }

        // INFO: ----------------------
        //         do rendering
        // ----------------------------

        let mut render_pass =
            render_context
                .encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Wireframe Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: render_context.surface_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

        // do render
        render_pass.set_pipeline(&wireframe_pipeline.inner.pipeline);
        render_pass.set_bind_group(0, &view_bind_group.bind_group, &[]);
        render_pass.set_bind_group(1, &wireframe_buffer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, wireframe_mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            wireframe_mesh.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(
            0..wireframe_mesh.index_count,
            0,
            0..wireframe_buffer.objects.len() as u32,
        );
    }
}
