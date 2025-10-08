use bevy_ecs::prelude::*;

use crate::game_world::ui::components::{Node, Parent, UiRoot};

#[derive(Component)]
pub struct UiDepth(pub f32);

/// A system that runs after layout to calculate the depth of each UI node.
pub fn compute_ui_depth_system(
    // Input (queries)
    nodes_to_update: Query<Entity, (With<Node>, Without<UiDepth>)>,
    parents_query: Query<&Parent>,
    root_query: Query<Entity, With<UiRoot>>,

    // Output (depth attachment component)
    mut commands: Commands,
) {
    // TODO: there is likely a more efficient alg here
    // but i'll have to think on it since we also need to consider
    // that we are only updated "dirty" nodes that lack depth
    if let Ok(root_entity) = root_query.single() {
        for entity in &nodes_to_update {
            let mut depth = 0.0;
            let mut current_entity = entity;

            // Traverse up the hierarchy to the rootkk
            while let Ok(parent) = parents_query.get(current_entity) {
                depth += 1.0;
                current_entity = parent.0;
                if current_entity == root_entity {
                    break;
                }
            }

            commands.entity(entity).insert(UiDepth(depth));
        }
    }
}
