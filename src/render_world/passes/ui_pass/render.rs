use crate::render_world::passes::{
    render_graph::{RenderContext, RenderNode},
    ui_pass::{
        prepare::UiRenderBatch,
        queue::{RenderPhase, UiPhaseItem},
        startup::{
            GlyphonAtlas, GlyphonCache, GlyphonFontSystem, GlyphonRenderer, GlyphonViewport,
            UiMaterialBuffer, UiObjectBuffer, UiPipeline,
        },
    },
};
use bevy_ecs::world::World;
use glyphon::{Buffer, Metrics, TextArea, TextBounds};

use super::{prepare::UiViewBindGroup, startup::ScreenQuadResource};

pub struct UiPassNode;
impl RenderNode for UiPassNode {
    fn run(&mut self, render_context: &mut RenderContext, world: &World) {
        // INFO: ---------------------------
        //         Resource fetching
        // ---------------------------------

        let ui_phase = world.get_resource::<RenderPhase<UiPhaseItem>>().unwrap();
        let pipeline = world.get_resource::<UiPipeline>().unwrap();
        let quad = world.get_resource::<ScreenQuadResource>().unwrap();
        let view_bind_group = world.get_resource::<UiViewBindGroup>().unwrap();
        let material_buffer = world.get_resource::<UiMaterialBuffer>().unwrap();
        let object_buffer = world.get_resource::<UiObjectBuffer>().unwrap();

        // Glyphon resources
        let font_system = world.get_resource::<GlyphonFontSystem>().unwrap();
        let cache = world.get_resource::<GlyphonCache>().unwrap();
        let atlas = world.get_resource::<GlyphonAtlas>().unwrap();
        let viewport = world.get_resource::<GlyphonViewport>().unwrap();
        let renderer = world.get_resource::<GlyphonRenderer>().unwrap();

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

        let mut is_panel_pipeline_set = false;

        for batch in &ui_phase.queue {
            match batch {
                UiRenderBatch::Panel(panel_batch) => {
                    if !is_panel_pipeline_set {
                        render_pass.set_pipeline(&pipeline.pipeline);
                        render_pass.set_bind_group(0, &view_bind_group.bind_group, &[]);
                        render_pass.set_vertex_buffer(0, quad.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(
                            quad.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.set_bind_group(2, &object_buffer.bind_group, &[]);
                        is_panel_pipeline_set = true;
                    }

                    let material_offset = panel_batch.material_index * material_buffer.stride;
                    render_pass.set_bind_group(1, &material_buffer.bind_group, &[material_offset]);
                    render_pass.draw_indexed(
                        0..quad.index_count,
                        0,
                        panel_batch.first_instance
                            ..panel_batch.first_instance + panel_batch.instance_count,
                    );
                }
                UiRenderBatch::Text(text_batch) => {
                    is_panel_pipeline_set = false;

                    let mut font_system = font_system.0.write().unwrap();

                    // Create buffers that live for the scope of this batch rendering
                    let buffers: Vec<Buffer> = text_batch
                        .texts
                        .iter()
                        .map(|text_kind| {
                            if let crate::render_world::extract::ui::UiElementKind::Text {
                                content,
                                bounds,
                                font_size,
                                align,
                                ..
                            } = text_kind
                            {
                                let mut buffer = Buffer::new(
                                    &mut font_system,
                                    Metrics::new(*font_size, *font_size),
                                );

                                let attrs =
                                    glyphon::Attrs::new().family(glyphon::Family::Name("Miracode"));
                                buffer.set_rich_text(
                                    &mut font_system,
                                    [(content.as_str(), attrs.clone())],
                                    &attrs,
                                    glyphon::Shaping::Advanced,
                                    Some(*align),
                                );

                                buffer.set_size(&mut font_system, Some(bounds.x), Some(bounds.y));

                                buffer
                            } else {
                                unreachable!();
                            }
                        })
                        .collect();

                    // Create TextAreas that borrow from the buffers
                    let text_areas: Vec<TextArea> = buffers
                        .iter()
                        .zip(text_batch.texts.iter())
                        .map(|(buffer, text_kind)| {
                            if let crate::render_world::extract::ui::UiElementKind::Text {
                                position,
                                bounds,
                                color,
                                ..
                            } = text_kind
                            {
                                let text_color = glyphon::Color::rgba(
                                    (color[0] * 255.0) as u8,
                                    (color[1] * 255.0) as u8,
                                    (color[2] * 255.0) as u8,
                                    (color[3] * 255.0) as u8,
                                );

                                TextArea {
                                    buffer,
                                    left: position.x,
                                    top: position.y,
                                    scale: 1.0,
                                    bounds: TextBounds {
                                        left: position.x as i32,
                                        top: position.y as i32,
                                        right: position.x as i32 + bounds.x as i32,
                                        bottom: position.y as i32 + bounds.y as i32,
                                    },
                                    default_color: text_color,
                                    custom_glyphs: &[],
                                }
                            } else {
                                unreachable!();
                            }
                        })
                        .collect();

                    renderer
                        .0
                        .write()
                        .unwrap()
                        .prepare(
                            &render_context.device,
                            &render_context.queue,
                            &mut font_system,
                            &mut atlas.0.write().unwrap(),
                            &mut viewport.0.write().unwrap(),
                            text_areas,
                            &mut cache.0.write().unwrap(),
                        )
                        .unwrap();

                    renderer
                        .0
                        .read()
                        .unwrap()
                        .render(
                            &atlas.0.read().unwrap(),
                            &viewport.0.read().unwrap(),
                            &mut render_pass,
                        )
                        .unwrap();
                }
            }
        }
    }
}
