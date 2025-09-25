use crate::ecs::components::ScreenTextComponent;
use crate::ecs::resources::{RenderQueueResource, WindowResource};
use crate::graphics::text_renderer::QueuedText;
use crate::graphics::GlyphonRenderer;
use bevy_ecs::prelude::{Query, Res, ResMut};
use glyphon::cosmic_text::{Attrs, Buffer, Color, Family, Metrics, Shaping};
use glyphon::TextBounds;
use tracing::debug;

pub fn screen_text_render_system(
    mut renderer: ResMut<GlyphonRenderer>,
    mut render_queue: ResMut<RenderQueueResource>,
    query: Query<&ScreenTextComponent>,
    window: Res<WindowResource>,
) {
    for component in query.iter() {
        let mut buffer = Buffer::new(
            &mut renderer.font_system,
            Metrics::new(component.font_size, component.font_size),
        );

        buffer.set_size(
            &mut renderer.font_system,
            Some(window.width as f32),
            Some(window.height as f32),
        );

        buffer.set_text(
            &mut renderer.font_system,
            &component.text,
            &Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );

        buffer.shape_until_scroll(&mut renderer.font_system, false);

        let queued_text = QueuedText {
            buffer,
            left: component.position.x,
            top: component.position.y,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: 1000,
                bottom: 1000,
            },
            default_color: Color::rgb(255, 130, 255),
        };

        debug!(target : "text_queue", "Text queued: {:?}", &component.text);

        // renderer.queue_text(queued_text);
        render_queue.add_screen_text(queued_text);
    }
}
