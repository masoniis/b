use crate::{
    core::graphics::renderpass::{ITextRenderPass, RenderPassContex},
    ecs_modules::graphics::{CameraUniformResource, RenderQueueResource},
    ecs_resources::{asset_storage::MeshAsset, AssetStorageResource},
};
use glyphon::{
    cosmic_text::{Attrs, Family, Metrics, Shaping},
    fontdb::Source,
    Buffer, Cache, FontSystem, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use wgpu::{Device, MultisampleState, Queue, TextureFormat};

pub struct TextRenderPass {
    pub renderer: TextRenderer,
    pub font_system: FontSystem,
    pub cache: SwashCache,
    pub atlas: TextAtlas,
    pub viewport: Viewport,
}

impl TextRenderPass {
    pub fn new(
        device: &Device,
        queue: &Queue,
        target_format: TextureFormat,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        // Loading font from assets
        let font_path = Path::new("assets/fonts/Miracode.ttf");
        let font_bytes = match fs::read(font_path) {
            Ok(bytes) => Arc::new(bytes),
            Err(e) => {
                panic!("Failed to load font file at {:?}: {}", font_path, e);
            }
        };

        let source = Source::Binary(font_bytes);
        let font_system = FontSystem::new_with_fonts(vec![source]);

        let cache = SwashCache::new();
        let viewport_cache = Cache::new(device);
        let mut viewport = Viewport::new(device, &viewport_cache);
        viewport.update(
            queue,
            glyphon::Resolution {
                width: size.width,
                height: size.height,
            },
        );
        let mut atlas = TextAtlas::new(device, queue, &viewport_cache, target_format);
        let renderer = TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);
        Self {
            font_system,
            cache,
            atlas,
            renderer,
            viewport,
        }
    }
}
impl ITextRenderPass for TextRenderPass {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_queue: &RenderQueueResource,
        _mesh_assets: &AssetStorageResource<MeshAsset>,
        _camera_uniform: &CameraUniformResource,
    ) {
        let mut buffers = Vec::new();
        for text in render_queue.get_screen_texts() {
            let mut buffer = Buffer::new(
                &mut self.font_system,
                Metrics::new(text.font_size, text.font_size),
            );
            buffer.set_text(
                &mut self.font_system,
                &text.text,
                &Attrs::new().family(Family::Name("Miracode")),
                Shaping::Advanced,
            );
            buffers.push(buffer);
        }
        let text_areas = buffers
            .iter()
            .zip(render_queue.get_screen_texts())
            .map(|(buffer, text)| TextArea {
                buffer: buffer,
                left: text.position.x,
                top: text.position.y,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: 1000,
                    bottom: 1000,
                },
                default_color: text.color,
                custom_glyphs: &[],
            })
            .collect::<Vec<_>>();
        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                text_areas,
                &mut self.cache,
            )
            .unwrap();
    }
    fn render<'a>(&'a self, encoder: &mut wgpu::CommandEncoder, context: RenderPassContex<'a>) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Text Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: context.view,
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
        self.renderer
            .render(&self.atlas, &self.viewport, &mut render_pass)
            .unwrap();
    }
}
