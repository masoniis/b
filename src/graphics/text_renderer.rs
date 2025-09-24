use bevy_ecs::prelude::Resource;
use glyphon::{
    Buffer, Cache, Color, FontSystem, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer,
    Viewport,
};
use wgpu::{Device, MultisampleState, Queue, TextureFormat};

pub struct QueuedText {
    pub buffer: Buffer,
    pub left: f32,
    pub top: f32,
    pub scale: f32,
    pub bounds: TextBounds,
    pub default_color: Color,
}

#[derive(Resource)]
pub struct GlyphonRenderer {
    pub renderer: TextRenderer,

    pub font_system: FontSystem,
    pub cache: SwashCache,
    pub atlas: TextAtlas,
    pub viewport: Viewport,

    queued_texts: Vec<QueuedText>,
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
            queued_texts: Vec::new(),
        }
    }

    pub fn queue_text(&mut self, text: QueuedText) {
        self.queued_texts.push(text);
    }

    pub fn prepare_texts(
        &mut self,
        device: &Device,
        queue: &Queue,
    ) -> Result<(), glyphon::PrepareError> {
        let text_areas = self
            .queued_texts
            .iter()
            .map(|text| TextArea {
                buffer: &text.buffer,
                left: text.left,
                top: text.top,
                scale: text.scale,
                bounds: text.bounds,
                default_color: text.default_color,
                custom_glyphs: &[],
            })
            .collect::<Vec<_>>();

        let result = self.renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            &self.viewport,
            text_areas,
            &mut self.cache,
        );

        self.queued_texts.clear();

        result
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> Result<(), glyphon::RenderError> {
        self.renderer
            .render(&self.atlas, &self.viewport, render_pass)
    }
}
