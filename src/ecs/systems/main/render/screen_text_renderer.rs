use crate::ecs::components::ScreenTextComponent;
use crate::ecs::resources::RenderQueueResource;
use crate::graphics::renderpass::text_renderpass::QueuedText;
use bevy_ecs::prelude::{Query, ResMut};
use tracing::debug;

pub fn screen_text_render_system(
    mut render_queue: ResMut<RenderQueueResource>,
    query: Query<&ScreenTextComponent>,
) {
    for component in query.iter() {
        let queued_text = QueuedText {
            text: component.text.clone(),
            position: component.position,
            color: component.color,
            font_size: component.font_size,
        };

        debug!(target : "text_queue", "Text queued: {:?}", &component.text);

        render_queue.add_screen_text(queued_text);
    }
}
