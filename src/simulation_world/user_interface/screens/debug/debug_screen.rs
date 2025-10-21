use crate::prelude::*;
use crate::simulation_world::user_interface::components::{
    Node, Size, Style, TextAlign, UiBackground, UiText,
};
use crate::simulation_world::user_interface::screens::spawn_root::UiRootNodeResource;
use bevy_ecs::prelude::*;
use bevy_ecs::relationship::RelatedSpawnerCommands;

// --- Marker Components ---

/// A marker component for all entities that are part of the diag UI.
#[derive(Component)]
pub struct DiagnosticsUiElementMarker;

/// A marker component for the FPS Counter text element.
#[derive(Component)]
pub struct FpsCounterTextElementMarker;

/// A marker component for the total mesh count text element.
#[derive(Component)]
pub struct MeshCountTextMarker;

/// A marker component for the total vertex count text element.
#[derive(Component)]
pub struct VertexCountTextMarker;

/// A marker component for the total triangle count text element.
#[derive(Component)]
pub struct IndexCountTextMarker;

/// A run condition that returns true if the diagnostic UI is currently spawned and visible.
pub fn diagnostic_ui_is_visible(query: Query<(), With<DiagnosticsUiElementMarker>>) -> bool {
    !query.is_empty()
}

/// Toggles the debug diagnostics UI by spawning or despawning it.
#[instrument(skip_all)]
pub fn toggle_debug_diagnostics_system(
    // Input
    root_node: Res<UiRootNodeResource>,
    query: Query<Entity, With<DiagnosticsUiElementMarker>>,
    // Output (toggling UI)
    mut commands: Commands,
) {
    if let Ok(ui_entity) = query.single() {
        info!("Despawning Diagnostic UI...");
        commands.entity(ui_entity).despawn();
    } else {
        spawn_diagnostic_ui(&mut commands, &root_node);
    }
}

/// Spawns the entire Diagnostic UI and attaches it to the persistent root node.
fn spawn_diagnostic_ui(commands: &mut Commands, root_node: &Res<UiRootNodeResource>) {
    info!("Spawning Diagnostic UI...");
    let root_entity = root_node.0;

    let diagnostic_ui_container = commands
        .spawn((
            DiagnosticsUiElementMarker,
            Node,
            Style {
                position: taffy::style::Position::Absolute,
                width: Size::Percent(100.0),
                height: Size::Percent(100.0),
                flex_direction: taffy::style::FlexDirection::Column,
                justify_content: Some(taffy::JustifyContent::Start),
                align_items: Some(taffy::AlignItems::Start),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            // Spawn the FPS counter on its own line
            spawn_single_text_line(
                parent,
                FpsCounterTextElementMarker,
                "FPS: 0".to_string(),
                [1.0, 1.0, 1.0, 1.0],
            );

            // Spawn the mesh stats all on one line
            spawn_mesh_stats_line(parent);
        })
        .id();

    commands
        .entity(root_entity)
        .add_child(diagnostic_ui_container);
}

/// A generic helper function to spawn one line of the diagnostic UI with a single text element.
fn spawn_single_text_line<M: Component>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    marker: M,
    text_content: String,
    color: [f32; 4],
) {
    parent
        .spawn((
            Node,
            Style {
                padding: 8.0,
                // These styles are for the background box of the line
                align_items: Some(taffy::style::AlignItems::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                ..Default::default()
            },
            UiBackground::SolidColor {
                color: [0.0, 0.0, 0.0, 0.33],
            },
        ))
        .with_children(|parent_box| {
            parent_box.spawn((
                marker,
                Node,
                Style::default(),
                UiText {
                    content: text_content,
                    font_size: 32.0,
                    color,
                    align: TextAlign::Center,
                },
            ));
        });
}

/// A specialized helper to spawn the multi-part mesh statistics line.
fn spawn_mesh_stats_line(parent: &mut RelatedSpawnerCommands<ChildOf>) {
    // This parent container uses FlexDirection::Row to align children horizontally.
    parent
        .spawn((
            Node,
            Style {
                padding: 8.0,
                flex_direction: taffy::style::FlexDirection::Row, // Arrange children horizontally
                align_items: Some(taffy::style::AlignItems::Center), // Center items vertically
                ..Default::default()
            },
            UiBackground::SolidColor {
                color: [0.0, 0.0, 0.0, 0.33],
            },
        ))
        .with_children(|line| {
            let font_size = 32.0;
            let align = TextAlign::Center;

            // Static label for Meshes
            line.spawn((
                Node,
                Style::default(),
                UiText {
                    content: "Meshes: ".to_string(),
                    font_size,
                    color: [0.7, 0.7, 0.7, 1.0],
                    align,
                },
            ));
            // Dynamic text for Mesh Count
            line.spawn((
                MeshCountTextMarker,
                Node,
                Style::default(),
                UiText {
                    content: "0".to_string(),
                    font_size,
                    color: [0.9, 0.6, 0.6, 1.0],
                    align,
                },
            ));

            // Static label for Vertices
            line.spawn((
                Node,
                Style::default(),
                UiText {
                    content: " Verts: ".to_string(),
                    font_size,
                    color: [0.7, 0.7, 0.7, 1.0],
                    align,
                },
            ));
            // Dynamic text for Vertex Count
            line.spawn((
                VertexCountTextMarker,
                Node,
                Style::default(),
                UiText {
                    content: "0".to_string(),
                    font_size,
                    color: [0.6, 0.8, 0.6, 1.0],
                    align,
                },
            ));

            // Static label for Triangles
            line.spawn((
                Node,
                Style::default(),
                UiText {
                    content: " Idxs: ".to_string(),
                    font_size,
                    color: [0.7, 0.7, 0.7, 1.0],
                    align,
                },
            ));
            // Dynamic text for Triangle Count
            line.spawn((
                IndexCountTextMarker,
                Node,
                Style::default(),
                UiText {
                    content: "0".to_string(),
                    font_size,
                    color: [0.6, 0.6, 0.9, 1.0],
                    align,
                },
            ));
        });
}
