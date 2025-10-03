use super::super::super::components::ScreenTextComponent;
use crate::{
    core::graphics::types::gpu_queues::QueuedText, ecs_modules::graphics::RenderQueueResource,
    ecs_modules::graphics::VisibilityComponent, prelude::*,
};
use bevy_ecs::prelude::*;

/// This system only runs when an entity's VisibilityComponent has changed.
pub fn handle_text_visibility_change_system(
    mut render_queue: ResMut<RenderQueueResource>,
    // Query for entities where VisibilityComponent has changed.
    // We also need its ScreenTextComponent to get the data if it becomes visible.
    query: Query<
        (Entity, &VisibilityComponent, &ScreenTextComponent),
        Changed<VisibilityComponent>,
    >,
) {
    for (entity, visibility, text_component) in query.iter() {
        match visibility {
            VisibilityComponent::Hidden => {
                // It's now hidden, so REMOVE it from the render queue.
                render_queue.remove_screen_text(&entity);
                debug!(target: "text_sync", "Text REMOVED for entity {:?} (now hidden)", entity);
            }
            VisibilityComponent::Visible => {
                // It's now visible, so ADD it to the render queue.
                let queued_text = QueuedText {
                    text: text_component.text.clone(),
                    position: text_component.position,
                    color: text_component.color,
                    font_size: text_component.font_size,
                };
                render_queue.add_screen_text(entity, queued_text);
                debug!(target: "text_sync", "Text ADDED for entity {:?} (now visible)", entity);
            }
        }
    }
}
