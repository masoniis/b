use crate::{
    game_world::{
        input::resources::WindowSizeResource,
        ui::{
            components::{self as game, CalculatedLayout},
            text::systems::FontSystemResource,
        },
    },
    prelude::*,
};
use bevy_ecs::prelude::*;
use std::collections::HashMap;

use taffy::{self, TaffyTree};

/// The Taffy tree that represents our UI layout.
///
/// It is to be instantiated as a NonSend resource because
/// Taffy is not Send/Sync.
pub struct UiLayoutTree(pub TaffyTree<Entity>);

impl Default for UiLayoutTree {
    fn default() -> Self {
        Self(TaffyTree::new())
    }
}

/// A map from our ECS entities to Taffy node IDs.
#[derive(Resource, Default)]
pub struct EntityToNodeMap(pub HashMap<Entity, taffy::NodeId>);

/// A system that syncs the ECS UI entities and their styles into the Taffy layout system.
///
/// This does not perform any layout calculations; it only ensures the Taffy tree reflects
/// the current state of the ECS UI entities.
pub fn sync_ui_to_taffy_system(
    mut commands: Commands,
    mut ui_tree: NonSendMut<UiLayoutTree>,
    query: Query<(
        Entity,
        &game::Style,
        Option<&game::Children>,
        Option<&game::UiText>,
    )>,
) {
    let mut entity_to_node = EntityToNodeMap::default();
    ui_tree.0.clear();

    // Convert entities into taffy nodes
    for (entity, style, _, maybe_text) in query.iter() {
        let taffy_style: taffy::style::Style = style.into();

        // If it has text we need to use custom measurement
        // so taffy can size it properly.
        let node_id = if maybe_text.is_some() {
            ui_tree.0.new_leaf_with_context(taffy_style, entity)
        } else {
            ui_tree.0.new_leaf(taffy_style)
        }
        .unwrap();
        entity_to_node.0.insert(entity, node_id);
    }

    // Build the taffy hierarchy
    for (entity, _, maybe_children, _) in query.iter() {
        if let Some(children) = maybe_children {
            let parent_node = entity_to_node.0[&entity];
            let child_nodes: Vec<taffy::NodeId> =
                children.0.iter().map(|e| entity_to_node.0[e]).collect();
            ui_tree.0.set_children(parent_node, &child_nodes).unwrap();
        }
    }

    commands.insert_resource(entity_to_node);
}

/// A function that computes the layout using Taffy and applies the results back to the ECS entities.
pub fn compute_and_apply_layout(world: &mut World) {
    let mut root_query = world.query_filtered::<Entity, With<game::UiRoot>>();
    let root_entity = match root_query.single(world) {
        Ok(entity) => entity,
        Err(e) => {
            error!("Error finding single UI Root: {:?}", e);
            return;
        }
    };

    let mut ui_tree = world.remove_non_send_resource::<UiLayoutTree>().unwrap();
    let entity_to_node = world.remove_resource::<EntityToNodeMap>().unwrap();
    let window_size = world.resource::<WindowSizeResource>();

    let root_node = entity_to_node.0[&root_entity];
    let viewport_size = taffy::Size {
        width: taffy::AvailableSpace::Definite(window_size.width as f32),
        height: taffy::AvailableSpace::Definite(window_size.height as f32),
    };

    ui_tree
        .0
        .compute_layout_with_measure(
            root_node,
            viewport_size,
            |known_dimensions, available_space, _node_id, node_context, _style| {
                if let Some(entity) = node_context {
                    let text = match world.get::<game::UiText>(*entity) {
                        Some(text) => text.clone(),
                        None => return taffy::Size::ZERO, // no text means no internal size constraint
                    };

                    return world.get_resource_mut::<FontSystemResource>().map_or(
                        taffy::Size::ZERO,
                        |mut font_system_res| {
                            measure_text_node(
                                known_dimensions,
                                available_space,
                                &text,
                                &mut font_system_res.font_system,
                            )
                        },
                    );
                }
                taffy::Size::ZERO // no text means no internal size constraint
            },
        )
        .unwrap();

    // Apply the results back to the ECS
    for (entity, node_id) in &entity_to_node.0 {
        let layout = ui_tree.0.layout(*node_id).unwrap();
        let calculated_layout = CalculatedLayout {
            position: Vec2::new(layout.location.x, layout.location.y),
            size: Vec2::new(layout.size.width, layout.size.height),
        };

        if let Ok(mut entity_mut) = world.get_entity_mut(*entity) {
            entity_mut.insert(calculated_layout);
        }
    }

    // Put the resources back
    world.insert_non_send_resource(ui_tree);
    world.insert_resource(entity_to_node);
}

// INFO: --------------------------
//         conversion utils
// --------------------------------

// Conversion from the gameworld component Style to Taffy's Style
impl From<&game::Style> for taffy::Style {
    fn from(value: &game::Style) -> Self {
        let to_dim = |size: game::Size| -> taffy::Dimension {
            match size {
                game::Size::Px(px) => taffy::Dimension::length(px),
                game::Size::Percent(percent) => taffy::Dimension::percent(percent / 100.0),
                game::Size::Auto => taffy::Dimension::auto(),
            }
        };

        taffy::style::Style {
            size: taffy::Size {
                width: to_dim(value.width),
                height: to_dim(value.height),
            },
            justify_content: value.justify_content,
            align_items: value.align_items,
            ..Default::default()
        }
    }
}

/// Measures the size of a text node using the provided FontSystem.
fn measure_text_node(
    _known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<taffy::AvailableSpace>,
    text: &game::UiText,
    font_system: &mut glyphon::FontSystem,
) -> taffy::Size<f32> {
    let max_width = match available_space.width {
        taffy::AvailableSpace::Definite(space) => space,
        taffy::AvailableSpace::MaxContent => f32::INFINITY,
        taffy::AvailableSpace::MinContent => 0.0,
    };

    let mut buffer = glyphon::Buffer::new(
        font_system,
        glyphon::Metrics::new(text.font_size, text.font_size),
    );

    buffer.set_size(font_system, Some(max_width), Some(f32::INFINITY));

    buffer.set_text(
        font_system,
        &text.content,
        &glyphon::Attrs::new().family(glyphon::Family::Name("Miracode")),
        glyphon::Shaping::Advanced,
    );

    buffer.shape_until_scroll(font_system, false);

    let (measured_width, measured_height) = (buffer.size().0, buffer.size().1);

    match (measured_width, measured_height) {
        (Some(w), Some(h)) => taffy::Size {
            width: w,
            height: h,
        },
        _ => taffy::Size::default(),
    }
}
