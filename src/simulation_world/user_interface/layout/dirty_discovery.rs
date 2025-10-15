use crate::{
    prelude::*,
    simulation_world::{
        input::resources::WindowSizeResource,
        user_interface::{
            components::{self as simulation},
            layout::{EntityToNodeMap, UiLayoutTree},
        },
    },
};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use taffy::{self};

// INFO: -------------------
//         Resources
// -------------------------

/// A marker resource that indicates whether the layout needs to be recomputed.
#[derive(Resource, Default, Deref, DerefMut, PartialEq)]
pub struct IsLayoutDirty(pub bool);

// INFO: -----------------------
//         Updating tree
// -----------------------------

/// A system that handles structural changes to the UI hierarchy (adding/removing nodes).
pub fn handle_structural_changes_system(
    // Input (added styles)
    add_query: Query<
        (Entity, &simulation::Style, Option<&simulation::UiText>),
        Added<simulation::Style>,
    >,
    mut removed_components: RemovedComponents<simulation::Node>,

    // Output (updating tree, marking dirty)
    mut ui_tree: NonSendMut<UiLayoutTree>,
    mut entity_to_node: ResMut<EntityToNodeMap>,
    mut is_dirty: ResMut<IsLayoutDirty>,
) {
    // Handle additions
    if !add_query.is_empty() {
        is_dirty.0 = true;
        for (entity, style, maybe_text) in &add_query {
            let taffy_style: taffy::style::Style = style.into();
            let new_node = if maybe_text.is_some() {
                ui_tree.new_leaf_with_context(taffy_style, entity)
            } else {
                ui_tree.new_leaf(taffy_style) // no context for measuremnet needed if no text
            }
            .unwrap();
            entity_to_node.insert(entity, new_node);
        }
    }

    // Handle removals
    if !removed_components.is_empty() {
        is_dirty.0 = true;
        for entity in removed_components.read() {
            if let Some(node_id) = entity_to_node.remove(&entity) {
                // Taffy's remove is smart and handles reparenting children to the grandparent
                ui_tree.remove(node_id).unwrap();
            }
        }
    }
}

/// A system that detects changes in the ECS hierarchy (the `Children` component)
/// and synchronizes it to the Taffy tree.
pub fn handle_hierarchy_changes_system(
    // Input (changed hierarchy)
    hierarchy_query: Query<(Entity, &Children), Changed<Children>>,
    entity_to_node: Res<EntityToNodeMap>,

    // Output (updated tree/map)
    mut ui_tree: NonSendMut<UiLayoutTree>,
    mut is_dirty: ResMut<IsLayoutDirty>,
) {
    if !hierarchy_query.is_empty() {
        is_dirty.0 = true;
        for (parent_entity, children) in &hierarchy_query {
            // Get the Taffy node for the parent entity.
            if let Some(&parent_node) = entity_to_node.get(&parent_entity) {
                // Get the Taffy nodes for all of the children.
                let child_nodes: Vec<taffy::NodeId> = children
                    .iter()
                    .filter_map(|child_entity| entity_to_node.get(&child_entity).copied())
                    .collect();

                // If the number of resolved child nodes matches the number of children,
                // it means all child nodes have been created and we can set the hierarchy.
                if child_nodes.len() == children.len() {
                    ui_tree.set_children(parent_node, &child_nodes).unwrap();
                } else {
                    // This can happen if this system runs before the `add` system
                    // has created the nodes for newly added children. System ordering will fix this.
                    warn!("Could not set children for {:?}; some child nodes were not yet in the Taffy tree.", parent_entity);
                }
            }
        }
    }
}

/// A system that detects changes in Style or UiText components and updates the Taffy tree.
pub fn update_changed_styles_system(
    // Input (node queries)
    entity_to_node: Res<EntityToNodeMap>,
    style_query: Query<(Entity, &simulation::Style), Changed<simulation::Style>>,
    text_query: Query<Entity, Changed<simulation::UiText>>,

    // Output (tree changes, marking dirty)
    mut ui_tree: NonSendMut<UiLayoutTree>,
    mut is_dirty: ResMut<IsLayoutDirty>,
) {
    // Update styles for nodes where the Style component changed
    for (entity, style) in &style_query {
        if let Some(node) = entity_to_node.get(&entity) {
            let taffy_style: taffy::style::Style = style.into();
            ui_tree.set_style(*node, taffy_style).unwrap();
            is_dirty.0 = true;
        }
    }

    // For text changes, we don't need to update the style, but we MUST tell Taffy
    // that the node's intrinsic size may have changed. `mark_dirty` does exactly this.
    for entity in &text_query {
        if let Some(node) = entity_to_node.get(&entity) {
            ui_tree.mark_dirty(*node).unwrap();
            is_dirty.0 = true;
        }
    }
}

pub fn handle_window_resize_system(
    window_size: Res<WindowSizeResource>,
    mut is_dirty: ResMut<IsLayoutDirty>,
) {
    if window_size.is_changed() {
        // If the window size is different from the last time this system ran,
        // the entire layout must be re-calculated.
        is_dirty.0 = true;
    }
}
