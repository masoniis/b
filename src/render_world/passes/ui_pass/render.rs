use crate::render_world::passes::{
    render_graph::{RenderContext, RenderNode},
    ui_pass::{
        prepare::{PreparedUiNodes, ScreenQuadResource, UiPipeline, UiViewBindGroup},
        queue::{RenderPhase, UiPhaseItem},
    },
};
use bevy_ecs::world::World;

pub struct UiPassNode;
impl RenderNode for UiPassNode {
    fn run(&mut self, render_context: &mut RenderContext, world: &World) {
        // INFO: ---------------------------
        //         Resource fetching
        // ---------------------------------

        let ui_phase = world
            .get_resource::<RenderPhase<UiPhaseItem>>()
            .expect("UiPhaseItem resource not found");
        let prepared_nodes = world
            .get_resource::<PreparedUiNodes>()
            .expect("PreparedUiNodes resource not found");
        let pipeline = world
            .get_resource::<UiPipeline>()
            .expect("UiPipeline resource not found");
        let quad = world
            .get_resource::<ScreenQuadResource>()
            .expect("GpuQuad resource not found");
        let view_bind_group = world
            .get_resource::<UiViewBindGroup>()
            .expect("UiViewBindGroup resource not found");

        // INFO: ----------------------
        //         Render logic
        // ----------------------------

        let mut render_pass =
            render_context
                .encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("UI Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: render_context.surface_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

        render_pass.set_pipeline(&pipeline.pipeline);
        render_pass.set_bind_group(0, &view_bind_group.bind_group, &[]);
        render_pass.set_vertex_buffer(0, quad.vertex_buffer.slice(..));
        render_pass.set_index_buffer(quad.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        for item in &ui_phase.queue {
            let prepared_node = &prepared_nodes.nodes[item.prepared_node_index];
            render_pass.set_bind_group(1, &prepared_node.bind_group, &[]);
            render_pass.draw_indexed(0..quad.index_count, 0, 0..1);
        }
    }
}
