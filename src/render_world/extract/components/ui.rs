use crate::game_world::ui::components::UiText;
use crate::game_world::ui::layout::compute_depth::UiDepth;
use crate::prelude::*;
use crate::{
    game_world::ui::components::{CalculatedLayout, Node, UiBackground},
    render_world::extract::ExtractComponent,
};
use bevy_ecs::prelude::*;

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
    },
}

/// A component that marks an entity as a renderable UI element.
#[derive(Component)]
pub struct RenderableUiElement {
    pub sort_key: f32,
    pub kind: UiElementKind,
}

#[derive(Component, Debug)]
pub struct ExtractedUiNode {
    pub layout: CalculatedLayout,
    pub material: UiBackground,
}

/// A struct that extracts UI node information for rendering.
pub struct UiNodeExtractor;

impl ExtractComponent for UiNodeExtractor {
    type Extracted = ExtractedUiNode;
    type QueryComponents = (&'static CalculatedLayout, &'static UiBackground);
    type QueryFilter = With<Node>;

    fn extract(
        _entity: Entity,
        (layout, material): (&CalculatedLayout, &UiBackground),
    ) -> Self::Extracted {
        ExtractedUiNode {
            layout: *layout,
            material: material.clone(),
        }
    }
}

pub struct UiPanelExtractor;

impl ExtractComponent for UiPanelExtractor {
    type Extracted = RenderableUiElement;

    // Query for layout, material, AND our new depth component
    type QueryComponents = (
        &'static CalculatedLayout,
        &'static UiBackground,
        &'static UiDepth,
    );

    // Ensure we only run this on Nodes
    type QueryFilter = With<Node>;

    fn extract(
        _entity: Entity,
        (layout, background, depth): (&CalculatedLayout, &UiBackground, &UiDepth),
    ) -> Self::Extracted {
        RenderableUiElement {
            sort_key: depth.0,
            kind: UiElementKind::Panel {
                position: layout.position,
                size: layout.size,
                color: match background {
                    UiBackground::SolidColor { color } => *color,
                    UiBackground::Image { color } => *color,
                },
            },
        }
    }
}

// --- Extractor for UI Text ---

pub struct UiTextExtractor;

impl ExtractComponent for UiTextExtractor {
    type Extracted = RenderableUiElement;
    // Query for layout, text, AND our new depth component
    type QueryComponents = (&'static CalculatedLayout, &'static UiText, &'static UiDepth);
    type QueryFilter = With<Node>;

    fn extract(
        _entity: Entity,
        (layout, text, depth): (&CalculatedLayout, &UiText, &UiDepth),
    ) -> Self::Extracted {
        RenderableUiElement {
            // Add a small bias to the sort key to ensure text always renders
            // on top of its parent panel (which will have the same integer depth).
            sort_key: depth.0 + 0.1,
            kind: UiElementKind::Text {
                content: text.content.clone(),
                position: layout.position,
                bounds: layout.size,
                font_size: text.font_size,
                color: text.color,
            },
        }
    }
}
