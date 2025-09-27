use super::super::ScreenTextComponent;
use crate::core::graphics::renderpass::QueuedText;
use crate::ecs_resources::RenderQueueResource;
use bevy_ecs::prelude::{Changed, Entity, Query, ResMut};
use std::collections::hash_map::Entry;
use tracing::debug;

pub fn changed_screen_text_system(
    mut render_queue: ResMut<RenderQueueResource>,
    query: Query<(Entity, &ScreenTextComponent), Changed<ScreenTextComponent>>,
) {
    for (entity, component) in query.iter() {
        match render_queue.get_screen_text_entry(entity) {
            // The entry already exists, so we just update it.
            Entry::Occupied(mut entry) => {
                let queued_text = entry.get_mut();
                queued_text.text = component.text.clone();
                queued_text.position = component.position;
                queued_text.color = component.color;
                queued_text.font_size = component.font_size;
            }
            // The entry does NOT exist, so we create it with the final values directly.
            Entry::Vacant(entry) => {
                entry.insert(QueuedText {
                    text: component.text.clone(),
                    position: component.position,
                    color: component.color,
                    font_size: component.font_size,
                });
            }
        }
        debug!(target: "text_sync", "Text synced for entity {:?}", entity);
    }
}
