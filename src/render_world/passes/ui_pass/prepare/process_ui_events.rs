use std::collections::hash_map::Entry;

use crate::render_world::passes::ui_pass::{
    extract::{ExtractedUiEvent, ExtractedUiEvents, UiElementKind},
    queue::UiElementCache,
};
use bevy_ecs::prelude::*;

/// A resource that tracks the type of UI changes that occurred in a frame.
#[derive(Resource, Default)]
pub struct UiChanges {
    pub structural_change_occured: bool,
    pub panel_content_change_occured: bool,
    pub text_content_change_occured: bool,
}

/// A system that processes extracted UI events, updates the `UiElementCache`,
/// and flags what kind of changes occurred.
pub fn process_ui_events_system(
    // Input (events from extract stage)
    mut extracted_events: ResMut<ExtractedUiEvents>,

    // Output
    mut element_cache: ResMut<UiElementCache>,
    mut ui_changes: ResMut<UiChanges>,
) {
    *ui_changes = UiChanges::default(); // new frame = new flags

    let process_event = |event: ExtractedUiEvent,
                         cache: &mut UiElementCache,
                         changes: &mut UiChanges| {
        match event {
            ExtractedUiEvent::AddOrUpdate(element) => {
                let entity = element.entity();
                match cache.elements.entry(entity) {
                    Entry::Occupied(mut entry) => {
                        let old_element = entry.get_mut();
                        if old_element.sort_key != element.sort_key {
                            changes.structural_change_occured = true;
                        } else if matches!(element.kind, UiElementKind::Panel { .. }) {
                            changes.panel_content_change_occured = true;
                        } else {
                            changes.text_content_change_occured = true;
                        }
                        *old_element = element;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(element);
                        changes.structural_change_occured = true;
                    }
                }
            }
            ExtractedUiEvent::Remove(entity) => {
                if cache.elements.remove(&entity).is_some() {
                    changes.structural_change_occured = true;
                }
            }
        }
    };

    for event in extracted_events.panel_events.drain(..) {
        process_event(event, &mut element_cache, &mut ui_changes);
    }
    for event in extracted_events.text_events.drain(..) {
        process_event(event, &mut element_cache, &mut ui_changes);
    }
}
