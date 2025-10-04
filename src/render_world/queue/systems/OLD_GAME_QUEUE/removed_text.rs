use crate::game_world::{graphics::RenderQueueResource, screen_text::ScreenTextComponent};
use bevy_ecs::prelude::{RemovedComponents, ResMut};
use tracing::{debug, warn};

pub fn removed_screen_text_system(
    mut render_queue: ResMut<RenderQueueResource>,
    mut removed: RemovedComponents<ScreenTextComponent>,
) {
    for entity in removed.read() {
        if render_queue.remove_screen_text(&entity).is_some() {
            debug!(target: "text_sync", "Text removed for entity {:?}", entity);
        } else {
            warn!(
                "Attempted to remove text for entity {:?}, but it was not in the render queue.",
                entity
            );
        }
    }
}
