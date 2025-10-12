use bevy_ecs::prelude::*;

use crate::render_world::extract::{
    extract_component::ExtractedBy,
    ui::{UiPanelExtractor, UiTextExtractor},
};

/// A run condition that returns true if the extract stage has produced any UI elements
/// to be rendered. This assumes that the system that runs will flush the extraction
/// queue, and as such, the next frame this condition will be false.
pub fn ui_was_extracted(
    extracted_panels: Res<ExtractedBy<UiPanelExtractor>>,
    extracted_texts: Res<ExtractedBy<UiTextExtractor>>,
) -> bool {
    // We only need to run if there are items to process.
    !extracted_panels.items.is_empty() || !extracted_texts.items.is_empty()
}
