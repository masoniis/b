use crate::game_world::{
    input::resources::WindowSizeResource,
    ui::components::{CalculatedLayout, Children, Node, Size, Style, UiRoot},
};
use crate::prelude::*;
use bevy_ecs::prelude::*;

/// The main layout system.
/// It finds all UI roots and starts the recursive layout calculation from them.
pub fn layout_solver_system(
    mut commands: Commands,
    window_size: Res<WindowSizeResource>,
    // Query for all root nodes to kick things off.
    root_query: Query<(Entity, &Style, &Children), (With<Node>, With<UiRoot>)>,
    // A query for all nodes that we will pass to the recursive helper.
    // This allows us to look up children's components without borrowing conflicts.
    all_nodes_query: Query<(&Style, &Children), With<Node>>,
) {
    let root_size = Vec2::new(window_size.width as f32, window_size.height as f32);

    for (root_entity, _style, _children) in root_query.iter() {
        let root_pos = Vec2::ZERO; // (top left corner of window)

        layout_node_recursive(
            &all_nodes_query,
            root_entity,
            root_size,
            root_pos,
            &mut commands,
        );
    }
}

/// Recursively calculates the layout for a node and all of its children.
fn layout_node_recursive(
    // Input
    ui_tree: &Query<(&Style, &Children), With<Node>>,
    entity: Entity,
    parent_size: Vec2,
    parent_pos: Vec2,

    // Output (spawning entities)
    commands: &mut Commands,
) {
    // Get the Style and Children for the current node
    let (style, children) = match ui_tree.get(entity) {
        Ok(data) => data,
        Err(_) => {
            warn!(
                "Entity {:?} is missing Style or Children components.",
                entity
            );
            return;
        }
    };

    // Calculate the node's size
    let width = match style.width {
        Size::Px(px) => px,
        Size::Percent(pct) => parent_size.x * (pct / 100.0),
        Size::Auto => parent_size.x,
    };
    let height = match style.height {
        Size::Px(px) => px,
        Size::Percent(pct) => parent_size.y * (pct / 100.0),
        Size::Auto => parent_size.y,
    };
    let calculated_size = Vec2::new(width, height);
    let calculated_pos = parent_pos;

    // Store the results as an entity and recurse any children
    commands.entity(entity).insert(CalculatedLayout {
        position: calculated_pos,
        size: calculated_size,
    });

    for &child_entity in children.0.iter() {
        layout_node_recursive(
            ui_tree,
            child_entity,
            calculated_size,
            calculated_pos,
            commands,
        );
    }
}
