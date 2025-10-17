use crate::simulation_world::user_interface::{
    components::Node, screens::spawn_root::UiRootNodeResource,
};
use bevy_ecs::prelude::*;
use tracing::instrument;

#[derive(Component, Debug)]
pub struct UiDepth(pub f32);

/// A system that runs after layout to calculate the depth of each UI node.
///
/// Depth is computed as integer values starting from 0 at the root.
#[instrument(skip_all, fields(name = "compute_ui_depth_system"))]
pub fn compute_ui_depth_system(
    // Input (queries)
    root: ResMut<UiRootNodeResource>,
    children_query: Query<&Children, With<Node>>,
    mut depth_query: Query<&mut UiDepth>,

    // Output (spawned entities)
    mut commands: Commands,
) {
    // let _span = info_span!("compute_ui_depth_system").entered();

    let root_entity = root.0;
    let root_children = if let Ok(children) = children_query.get(root_entity) {
        children
    } else {
        return;
    };

    // Start the traversal from the root entity.
    commands.entity(root_entity).insert(UiDepth(0.0));

    // Recursively apply depth to all children.
    // NOTE: We now iterate directly over `root_children` thanks to Bevy's `Deref` implementation.
    for &child_entity in root_children {
        apply_depth_recursively(
            &mut commands,
            &children_query,
            &mut depth_query,
            child_entity,
            1.0, // initial depth for children of the root
        );
    }
}

/// A recursive helper function to traverse the UI tree and apply depth.
fn apply_depth_recursively(
    // Output (inserted depth component)
    commands: &mut Commands,

    // Input
    children_query: &Query<&Children, With<Node>>,
    depth_query: &mut Query<&mut UiDepth>,
    current_entity: Entity,
    current_depth: f32,
) {
    // do an uptate or insert of the depth component
    if let Ok(mut depth_component) = depth_query.get_mut(current_entity) {
        if depth_component.0 != current_depth {
            depth_component.0 = current_depth;
        }
    } else {
        commands
            .entity(current_entity)
            .insert(UiDepth(current_depth));
    }

    // If this node has children, recurse into them.
    if let Ok(children) = children_query.get(current_entity) {
        // NOTE: We also iterate directly over `children` here.
        for &child_entity in children {
            apply_depth_recursively(
                commands,
                children_query,
                depth_query,
                child_entity,
                current_depth + 1.0,
            );
        }
    }
}
