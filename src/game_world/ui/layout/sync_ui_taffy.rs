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
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use taffy::{self, TaffyTree};

// INFO: -------------------
//         Resources
// -------------------------

/// The Taffy tree that represents the UI layout.
///
/// It is to be instantiated as a NonSend resource because
/// Taffy is not Send/Sync, unfortunately.
pub struct UiLayoutTree(pub TaffyTree<Entity>);

impl Default for UiLayoutTree {
    fn default() -> Self {
        Self(TaffyTree::new())
    }
}

impl Deref for UiLayoutTree {
    type Target = TaffyTree<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UiLayoutTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A map from our ECS entities to Taffy node IDs.
#[derive(Resource, Default)]
pub struct EntityToNodeMap(pub HashMap<Entity, taffy::NodeId>);

impl Deref for EntityToNodeMap {
    type Target = HashMap<Entity, taffy::NodeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityToNodeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// INFO: -----------------
//         Systems
// -----------------------

/// A system that synchronizes the UI entity hierarchy from the ECS into the Taffy layout tree.
///
/// This system does not perform any layout computations. It solely ensures that Taffy has all
/// the proper nodes that the ECS world provides.
pub fn sync_ui_to_taffy_system(
    mut commands: Commands,
    mut ui_tree: NonSendMut<UiLayoutTree>,
    query: Query<(
        Entity,
        Option<&Children>,
        &game::Style,
        Option<&game::UiText>,
    )>,
) {
    let mut entity_to_node = EntityToNodeMap::default();
    ui_tree.clear();

    debug!(target: "ui_efficiency", "Syncing UI entities to Taffy...");

    // First pass: create all Taffy nodes for each entity.
    for (entity, _, style, maybe_text) in query.iter() {
        let taffy_style: taffy::style::Style = style.into();

        // If the entity has text, it needs a custom measure function.
        // We provide the entity as "context" so the layout computation step
        // can query for the text content and font system to measure it.
        let node_id = if maybe_text.is_some() {
            ui_tree.new_leaf_with_context(taffy_style, entity)
        } else {
            ui_tree.new_leaf(taffy_style)
        }
        .unwrap();
        entity_to_node.insert(entity, node_id);
    }

    // Second pass: build the hierarchy within Taffy by connecting nodes.
    for (entity, maybe_children, _, _) in query.iter() {
        if let Some(children) = maybe_children {
            let parent_node = entity_to_node[&entity];
            let child_nodes: Vec<taffy::NodeId> =
                children.iter().map(|e| entity_to_node[&e]).collect();
            ui_tree.set_children(parent_node, &child_nodes).unwrap();
        }
    }

    // Insert the map as a resource for the layout computation system to use.
    commands.insert_resource(entity_to_node);
}

/// Since taffy only computes position *relative to the parent*, we need to
/// recurse the hierarchy to create the absolute screen position for nodes.
///
/// Then the final absolute position and size is inserted into the ECS world.
fn apply_layouts_recursively(
    world: &mut World,
    ui_tree: &TaffyTree<Entity>,
    entity_to_node_map: &EntityToNodeMap,
    entity: Entity,
    parent_absolute_pos: Vec2,
) {
    // Get the taffy node for the current entity, do nothing if it doesn't exist in the map.
    let Some(node_id) = entity_to_node_map.get(&entity) else {
        return;
    };
    let Ok(layout) = ui_tree.layout(*node_id) else {
        return;
    };

    // Calculate the absolute position using the position relative to the parent
    let relative_pos = Vec2::new(layout.location.x, layout.location.y);
    let absolute_pos = parent_absolute_pos + relative_pos;

    let calculated_layout = CalculatedLayout {
        position: absolute_pos,
        size: Vec2::new(layout.size.width, layout.size.height),
    };

    // Log the final absolute layout for debugging
    if world.get::<game::UiText>(entity).is_some() {
        debug!(
            target: "ui_layout",
            "[Layout] Text Entity {:?}: abs_pos=({},{}), size=({},{})",
            entity,
            absolute_pos.x,
            absolute_pos.y,
            calculated_layout.size.x,
            calculated_layout.size.y
        );
    } else if world.get::<game::UiRoot>(entity).is_some() {
        debug!(
            target: "ui_layout",
            "[Layout] Root Entity {:?}: abs_pos=({},{}), size=({},{})",
            entity,
            absolute_pos.x,
            absolute_pos.y,
            calculated_layout.size.x,
            calculated_layout.size.y
        );
    } else {
        debug!(
            target: "ui_layout",
            "[Layout] UI Entity {:?}: abs_pos=({},{}), size=({},{})",
            entity,
            absolute_pos.x,
            absolute_pos.y,
            calculated_layout.size.x,
            calculated_layout.size.y
        );
    }

    // Insert the component with the absolute layout
    if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
        entity_mut.insert(calculated_layout);
    }

    // Recurse for children.
    let children: Option<Vec<Entity>> = world.get::<Children>(entity).map(|c| c.iter().collect());
    if let Some(children_vec) = children {
        for child_entity in children_vec {
            apply_layouts_recursively(
                world,
                ui_tree,
                entity_to_node_map,
                child_entity,
                absolute_pos,
            );
        }
    }
}

/// A system that computes the layout using Taffy and applies the results back to the ECS entities.
pub fn compute_and_apply_layout_system(world: &mut World) {
    debug!(target: "ui_efficiency", "Recomputing the UI layout...");

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

    let root_node = entity_to_node[&root_entity];
    let viewport_size = taffy::Size {
        width: taffy::AvailableSpace::Definite(window_size.width as f32),
        height: taffy::AvailableSpace::Definite(window_size.height as f32),
    };

    ui_tree
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

    apply_layouts_recursively(
        world,
        &ui_tree,
        &entity_to_node,
        root_entity,
        Vec2::ZERO, // root "parent" is the screen at (0,0)
    );

    // Put the resources back
    world.insert_non_send_resource(ui_tree);
    world.insert_resource(entity_to_node);
}

// INFO: --------------------------
//         conversion utils
// --------------------------------

/// Conversion from the gameworld component Style to Taffy's Style
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
