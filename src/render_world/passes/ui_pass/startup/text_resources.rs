use crate::render_world::resources::GraphicsContextResource;
use bevy_ecs::prelude::*;
use glyphon::{
    fontdb::Source,
    Cache, FontSystem, SwashCache, TextAtlas, TextRenderer, Viewport,
};
use std::sync::{Arc, RwLock};

#[derive(Resource)]
pub struct GlyphonFontSystem(pub RwLock<FontSystem>);

#[derive(Resource)]
pub struct GlyphonCache(pub RwLock<SwashCache>);

#[derive(Resource)]
pub struct GlyphonAtlas(pub RwLock<TextAtlas>);

#[derive(Resource)]
pub struct GlyphonViewport(pub RwLock<Viewport>);

#[derive(Resource)]
pub struct GlyphonRenderer(pub RwLock<TextRenderer>);

pub fn setup_glyphon_resources(
    mut commands: Commands,
    gfx: Res<GraphicsContextResource>,
) {
    let font_bytes = include_bytes!("../../../../../assets/fonts/Miracode.ttf");
    let source = Source::Binary(Arc::new(font_bytes));
    let font_system = FontSystem::new_with_fonts(vec![source]);

    let cache = SwashCache::new();
    let viewport_cache = Cache::new(&gfx.context.device);
    let mut viewport = Viewport::new(&gfx.context.device, &viewport_cache);
    viewport.update(
        &gfx.context.queue,
        glyphon::Resolution {
            width: gfx.context.config.width,
            height: gfx.context.config.height,
        },
    );
    let mut atlas = TextAtlas::new(
        &gfx.context.device,
        &gfx.context.queue,
        &viewport_cache,
        gfx.context.config.format,
    );
    let renderer = TextRenderer::new(&mut atlas, &gfx.context.device, wgpu::MultisampleState::default(), None);

    commands.insert_resource(GlyphonFontSystem(RwLock::new(font_system)));
    commands.insert_resource(GlyphonCache(RwLock::new(cache)));
    commands.insert_resource(GlyphonAtlas(RwLock::new(atlas)));
    commands.insert_resource(GlyphonViewport(RwLock::new(viewport)));
    commands.insert_resource(GlyphonRenderer(RwLock::new(renderer)));
}
