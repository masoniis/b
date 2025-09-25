use crate::{
    ecs::resources::{
        asset_storage::MeshAsset, AssetStorageResource, CameraUniformResource, RenderQueueResource,
    },
    graphics::{rendercore::WebGpuRenderer, renderpass::render_pass::RenderPass},
};
use glyphon::{
    cosmic_text::{Attrs, Family, Metrics, Shaping},
    Buffer, Cache, Color, FontSystem, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer,
    Viewport,
};
use wgpu::{Device, MultisampleState, Queue, TextureFormat};

pub struct QueuedText {
    pub text: String,
    pub position: glam::Vec2,
    pub color: Color,
    pub font_size: f32,
}

pub struct TextRenderPass {
    pub renderer: TextRenderer,

    pub font_system: FontSystem,
    pub cache: SwashCache,
    pub atlas: TextAtlas,
    pub viewport: Viewport,
}

impl TextRenderPass {
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
}

impl RenderPass for TextRenderPass {
    /// Method to run just before the rendering phase begins.
    /// Necessary for some renderpasses that require preprocessing
    /// and can be used to create buffers as well.
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
                &Attrs::new().family(Family::SansSerif),
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

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        _renderer: &mut WebGpuRenderer,
        _render_queue: &RenderQueueResource,
        _mesh_assets: &AssetStorageResource<MeshAsset>,
        _camera_uniform: &CameraUniformResource,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Text Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // LOAD the contents of the previous pass
                    load: wgpu::LoadOp::Load,
                    // Store the results of this pass (the text on top of the scene)
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            // No depth buffer for the UI pass
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.renderer
            .render(&self.atlas, &self.viewport, &mut render_pass)
            .unwrap();
    }
}
