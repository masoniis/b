use crate::prelude::*;
use crate::simulation_world::user_interface::components::{
    Node, Size, Style, TextAlign, UiBackground, UiText,
};
use crate::simulation_world::user_interface::screens::spawn_root::UiRootNodeResource;
use bevy_ecs::prelude::*;
use bevy_ecs::relationship::RelatedSpawnerCommands;

/// A marker component for all entities that are part of the diag UI.
#[derive(Component)]
pub struct DiagnosticsUiElementMarker;

/// A marker component for the FPS Counter text element.
#[derive(Component)]
pub struct FpsCounterTextElementMarker;

/// A marker component for the Mesh Counter text element.
#[derive(Component)]
pub struct MeshCounterTextElementMarker;

/// A run condition that returns true if the diagnostic UI is currently spawned and visible.
pub fn diagnostic_ui_is_visible(query: Query<(), With<DiagnosticsUiElementMarker>>) -> bool {
    !query.is_empty()
}

/// Spawns the FPS Counter UI and attaches it to the persistent root node.
fn spawn_diagnostic_ui(commands: &mut Commands, root_node: &Res<UiRootNodeResource>) {
    info!("Spawning Diagnostic UI...");
    let root_entity = root_node.0;

    // A generic helper function to spawn one line of the diagnostic UI.
    fn spawn_diagnostic_line<M: Component>(
        parent: &mut RelatedSpawnerCommands<ChildOf>,
        marker: M,
        text_content: String,
    ) {
        parent
            .spawn((
                Node,
                Style {
                    padding: 8.0,
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
                    Style {
                        ..Default::default()
                    },
                    UiText {
                        content: text_content,
                        font_size: 32.0,
                        color: [1.0, 1.0, 1.0, 1.0],
                        align: TextAlign::Center,
                    },
                ));
            });
    }

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
            {
                spawn_diagnostic_line(parent, FpsCounterTextElementMarker, "FPS: 0".to_string());
            }
            {
                spawn_diagnostic_line(
                    parent,
                    MeshCounterTextElementMarker,
                    "Mesh count: ".to_string(),
                );
            }
        })
        .id();

    commands
        .entity(root_entity)
        .add_child(diagnostic_ui_container);
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
