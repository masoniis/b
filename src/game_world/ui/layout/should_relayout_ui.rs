use crate::game_world::{
    input::resources::WindowSizeResource,
    ui::components::{Node, Style, UiText},
};
use bevy_ecs::prelude::*;

/// Run condition that returns `true` if the UI layout needs to be recomputed.
///
/// This checks for changes in layout-affecting components (`Style`, `UiText`),
/// hierarchy changes (`Children`), or the addition of new UI nodes.
pub fn should_relayout_ui(
    // Check for changed components that affect layout
    q_layout_components: Query<(), Or<(Changed<Style>, Changed<UiText>)>>,

    // Check for newly added UI nodes
    q_added_nodes: Query<(), Added<Node>>,

    // Check for hierarchy changes (adding/removing children)
    q_hierarchy: Query<(), (Changed<Children>, With<Node>)>,

    // Check for deletions of nodes
    removed_node: RemovedComponents<Node>,

    // Check for the viewport changing
    window_size: Res<WindowSizeResource>,
) -> bool {
    // If any of these queries are not empty, or if a parent was removed from a node,
    // it means something has changed that requires a relayout.
    !q_layout_components.is_empty()
        || !q_added_nodes.is_empty()
        || !q_hierarchy.is_empty()
        || !removed_node.is_empty()
        || window_size.is_changed()
}
