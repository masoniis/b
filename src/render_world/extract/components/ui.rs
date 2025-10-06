use crate::{
    game_world::ui::components::{CalculatedLayout, Node, UiMaterial},
    render_world::extract::ExtractComponent,
};
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct ExtractedUiNode {
    pub layout: CalculatedLayout,
    pub material: UiMaterial,
}

/// A marker struct that defines how to extract UI nodes.
pub struct UiNodeExtractor;

impl ExtractComponent for UiNodeExtractor {
    type Extracted = ExtractedUiNode;
    type QueryComponents = (&'static CalculatedLayout, &'static UiMaterial);
    type QueryFilter = With<Node>;

    fn extract(
        _entity: Entity,
        (layout, material): (&CalculatedLayout, &UiMaterial),
    ) -> Self::Extracted {
        ExtractedUiNode {
            layout: *layout,
            material: material.clone(),
        }
    }
}
