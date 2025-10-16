use crate::prelude::*;
use crate::render_world::global_extract::utils::run_extract_schedule::SimulationWorld;
use crate::simulation_world::user_interface::components::{CalculatedLayout, Node, UiBackground};
use crate::simulation_world::user_interface::{
    components::TextAlign, components::UiText, layout::compute_depth::UiDepth,
};
use bevy_ecs::prelude::*;

#[derive(Clone, Debug)]
pub enum UiElementKind {
    Panel {
        position: Vec2,
        size: Vec2,
        color: [f32; 4],
    },
    Text {
        content: String,
        position: Vec2,
        bounds: Vec2,
        font_size: f32,
        color: [f32; 4],
        align: glyphon::cosmic_text::Align,
    },
}

/// A component that marks an entity as a renderable UI element.
#[derive(Component, Clone)]
pub struct RenderableUiElement {
    entity_key: Entity,
    pub sort_key: f32,
    pub kind: UiElementKind,
}

impl ContainsEntity for RenderableUiElement {
    fn entity(&self) -> Entity {
        self.entity_key
    }
}

/// A resource holding extracted UI events for rendering.
#[derive(Resource, Default)]
pub struct ExtractedUiEvents {
    pub panel_events: Vec<ExtractedUiEvent>,
    pub text_events: Vec<ExtractedUiEvent>,
}

pub enum ExtractedUiEvent {
    AddOrUpdate(RenderableUiElement),
    Remove(Entity),
}

pub fn extract_ui_events_system(
    // Input (world to query)
    mut simulation_world: ResMut<SimulationWorld>,

    // Output (pushed events)
    mut extracted_events: ResMut<ExtractedUiEvents>,
) {
    extracted_events.panel_events.clear();
    extracted_events.text_events.clear();

    // INFO: ------------------------------
    //         Extract panel events
    // ------------------------------------

    // removed panel events
    let panel_removal_id = simulation_world.val.component_id::<UiBackground>().unwrap();
    for entity in simulation_world.val.removed_with_id(panel_removal_id) {
        extracted_events
            .panel_events
            .push(ExtractedUiEvent::Remove(entity));
    }

    // changed panel events
    let mut panel_query = simulation_world
        .val
        .query_filtered::<(Entity, &CalculatedLayout, &UiBackground, &UiDepth), Or<(
            Changed<CalculatedLayout>,
            Changed<UiBackground>,
            Changed<UiDepth>,
            Added<Node>,
        )>>();

    for (entity, layout, background, depth) in panel_query.iter(&simulation_world.val) {
        let renderable_element = RenderableUiElement {
            entity_key: entity,
            sort_key: depth.0,
            kind: UiElementKind::Panel {
                position: layout.position,
                size: layout.size,
                color: match background {
                    UiBackground::SolidColor { color } => *color,
                    UiBackground::Image { color } => *color,
                },
            },
        };

        extracted_events
            .panel_events
            .push(ExtractedUiEvent::AddOrUpdate(renderable_element));
    }

    // INFO: -----------------------------
    //         Extract text events
    // -----------------------------------

    // removed text events
    let text_removal_id = simulation_world.val.component_id::<UiText>().unwrap();
    for entity in simulation_world.val.removed_with_id(text_removal_id) {
        extracted_events
            .text_events
            .push(ExtractedUiEvent::Remove(entity));
    }

    // changed text events
    let mut text_query = simulation_world
        .val
        .query_filtered::<(Entity, &CalculatedLayout, &UiText, &UiDepth), Or<(
            Changed<CalculatedLayout>,
            Changed<UiText>,
            Changed<UiDepth>,
            Added<Node>,
        )>>();

    for (entity, layout, text, depth) in text_query.iter(&simulation_world.val) {
        let renderable_element = RenderableUiElement {
            entity_key: entity,
            sort_key: depth.0 + 0.1, // depth bias to ensure text of same-depth renders above panels
            kind: UiElementKind::Text {
                content: text.content.clone(),
                position: layout.position,
                bounds: layout.size,
                font_size: text.font_size,
                color: text.color,
                align: match text.align {
                    TextAlign::Start => glyphon::cosmic_text::Align::Left,
                    TextAlign::Center => glyphon::cosmic_text::Align::Center,
                    TextAlign::End => glyphon::cosmic_text::Align::Right,
                    TextAlign::Justified => glyphon::cosmic_text::Align::Justified,
                },
            },
        };
        extracted_events
            .text_events
            .push(ExtractedUiEvent::AddOrUpdate(renderable_element));
    }
}
