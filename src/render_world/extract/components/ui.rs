use crate::game_world::ui::components::UiText;
use crate::game_world::ui::layout::compute_depth::UiDepth;
use crate::prelude::*;
use crate::{
    game_world::ui::components::{CalculatedLayout, Node, UiBackground},
    render_world::extract::ExtractComponent,
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
    },
}

/// A component that marks an entity as a renderable UI element.
#[derive(Component)]
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

/// A struct that extracts UI panel information for rendering.
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

    type ChangeTracked = Or<(
        Changed<CalculatedLayout>,
        Changed<UiBackground>,
        Changed<UiDepth>,
        Added<Node>,
    )>;

    fn extract(
        entity: Entity,
        (layout, background, depth): (&CalculatedLayout, &UiBackground, &UiDepth),
    ) -> Self::Extracted {
        RenderableUiElement {
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
        }
    }
}

/// A struct that extracts UI text information for rendering.
pub struct UiTextExtractor;

impl ExtractComponent for UiTextExtractor {
    type Extracted = RenderableUiElement;
    type QueryComponents = (&'static CalculatedLayout, &'static UiText, &'static UiDepth);

    type QueryFilter = With<Node>;
    type ChangeTracked = Or<(
        Changed<CalculatedLayout>,
        Changed<UiText>,
        Changed<UiDepth>,
        Added<Node>,
    )>;

    fn extract(
        entity: Entity,
        (layout, text, depth): (&CalculatedLayout, &UiText, &UiDepth),
    ) -> Self::Extracted {
        RenderableUiElement {
            entity_key: entity,
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
