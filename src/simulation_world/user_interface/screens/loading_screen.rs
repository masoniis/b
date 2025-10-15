use crate::prelude::*;
use crate::simulation_world::user_interface::components::{
    Node, Size, Style, TextAlign, UiBackground, UiText,
};
use crate::simulation_world::user_interface::screens::spawn_root::UiRootNodeResource;
use bevy_ecs::prelude::*;

/// A marker component for all entities that are part of the Loading UI screen.
#[derive(Component)]
pub struct LoadingUiElement;

/// Spawns the Loading UI and attaches it to the persistent root node.
pub fn spawn_loading_ui_system(mut commands: Commands, root_node: Res<UiRootNodeResource>) {
    info!("Spawning Loading UI...");

    let root_entity = root_node.0;

    let loading_screen = commands
        .spawn((
            LoadingUiElement,
            Node,
            UiBackground::SolidColor {
                color: [0.1, 0.1, 0.1, 0.8],
            },
            Style {
                width: Size::Percent(100.0),
                height: Size::Percent(100.0),
                justify_content: Some(taffy::JustifyContent::Center),
                align_items: Some(taffy::AlignItems::Center),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node,
                    Style {
                        width: Size::Percent(33.33),
                        height: Size::Percent(25.0),
                        justify_content: Some(taffy::JustifyContent::Center),
                        align_items: Some(taffy::AlignItems::Center),
                        ..Default::default()
                    },
                    UiBackground::SolidColor {
                        color: [0.1, 0.3, 0.1, 0.5],
                    },
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Node,
                        Style {
                            width: Size::Percent(100.0),
                            height: Size::Px(48.0),
                            ..Default::default()
                        },
                        UiText {
                            content: "Loading...".to_string(),
                            font_size: 48.0,
                            color: [1.0, 1.0, 1.0, 1.0],
                            align: TextAlign::Center,
                        },
                    ));
                });
        })
        .id();

    commands.entity(root_entity).add_child(loading_screen);
}

/// Clears the Loading UI by finding and despawning all entities with the `LoadingUiElement` marker.
pub fn despawn_loading_ui_system(
    // Input (ui entity)
    loading_ui_query: Query<Entity, With<LoadingUiElement>>,

    // Output (despawn UI)
    mut commands: Commands,
) {
    info!("Despawning Loading UI...");

    for entity in loading_ui_query.iter() {
        commands.entity(entity).despawn();
    }
}
