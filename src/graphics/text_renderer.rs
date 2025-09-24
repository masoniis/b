use bevy_ecs::prelude::Resource;
use glyphon::{Cache, FontSystem, SwashCache, TextArea, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, MultisampleState, Queue, TextureFormat};

#[derive(Resource)]
pub struct GlyphonRenderer {
    pub renderer: TextRenderer,

    pub font_system: FontSystem,
    pub cache: SwashCache,
    pub atlas: TextAtlas,
    pub viewport: Viewport,
}

impl GlyphonRenderer {
    pub fn new(device: &Device, queue: &Queue, target_format: TextureFormat) -> Self {
        let font_system = FontSystem::new();
        let cache = SwashCache::new();
        let viewport_cache = Cache::new(device);
        let viewport = Viewport::new(device, &viewport_cache);
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

    pub fn prepare_texts(
        &mut self,
        device: &Device,
        queue: &Queue,
        texts: Vec<TextArea>,
    ) -> Result<(), glyphon::PrepareError> {
        self.renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            &self.viewport,
            texts,
            &mut self.cache,
        )
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> Result<(), glyphon::RenderError> {
        self.renderer
            .render(&self.atlas, &self.viewport, render_pass)
    }
}
