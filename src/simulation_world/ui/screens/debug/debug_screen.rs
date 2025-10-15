use crate::prelude::*;
use crate::simulation_world::ui::components::{Node, Size, Style, TextAlign, UiBackground, UiText};
use crate::simulation_world::ui::screens::spawn_root::UiRootNodeResource;
use bevy_ecs::prelude::*;

/// A marker component for all entities that are part of the diag UI.
#[derive(Component)]
pub struct DiagnosticsUiElementMarker;

/// A marker component for the FPS Counter text element.
#[derive(Component)]
pub struct FpsCounterTextElementMarker;

/// Spawns the FPS Counter UI and attaches it to the persistent root node.
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
                justify_content: Some(taffy::JustifyContent::FlexStart),
                align_items: Some(taffy::AlignItems::FlexStart),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node,
                    Style {
                        padding: 8.0,
                        ..Default::default()
                    },
                    UiBackground::SolidColor {
                        color: [0.0, 0.0, 0.0, 0.33],
                    },
                ))
                .with_children(|parent_box| {
                    parent_box.spawn((
                        Node,
                        Style {
                            ..Default::default()
                        },
                        UiText {
                            content: "FPS: 60".to_string(),
                            font_size: 32.0,
                            color: [1.0, 1.0, 1.0, 1.0],
                            align: TextAlign::Center,
                        },
                    ));
                });
        })
        .id();

    commands
        .entity(root_entity)
        .add_child(diagnostic_ui_container);
}

/// Toggles the debug diagnostics UI by spawning or despawning it.
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
