use super::super::super::components::ScreenTextComponent;
use crate::game_world::graphics::RenderQueueResource;
use crate::{game_world::graphics::VisibilityComponent, prelude::*};
use bevy_ecs::prelude::*;
use std::collections::hash_map::Entry;

pub fn update_visible_text_system(
    mut render_queue: ResMut<RenderQueueResource>,
    // 1. ADD `&VisibilityComponent` to the tuple of data we query for.
    //    The `Changed` filter still applies to `ScreenTextComponent`.
    query: Query<
        (Entity, &ScreenTextComponent, &VisibilityComponent),
        Changed<ScreenTextComponent>,
    >,
) {
    // 2. De-structure the new tuple in the for loop.
    for (entity, component, visibility) in query.iter() {
        // 3. Add an `if` statement to check the VALUE of the visibility component.
        if *visibility == VisibilityComponent::Visible {
            // All of your original update logic now goes inside this `if` block.
            if let Entry::Occupied(mut entry) = render_queue.get_screen_text_entry(entity) {
                let queued_text = entry.get_mut();
                queued_text.text = component.text.clone();
                queued_text.position = component.position;
                queued_text.color = component.color;
                queued_text.font_size = component.font_size;
                debug!(target: "text_sync", "Visible text UPDATED for entity {:?}", entity);
            }
        }
    }
}
