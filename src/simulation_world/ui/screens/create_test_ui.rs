use crate::prelude::*;
use crate::simulation_world::ui::components::{Node, Size, Style, TextAlign, UiBackground, UiText};
use crate::simulation_world::ui::screens::spawn_root::UiRootNodeResource;
use bevy_ecs::prelude::*;

/// A marker component for all entities that are part of the test UI screen.
#[derive(Component)]
pub struct TestUiElement;

/// Spawns a test UI by attaching it to the persistent root node.
pub fn create_test_ui_system(mut commands: Commands, root_node: Res<UiRootNodeResource>) {
    info!("Spawning test UI");

    // Get the persistent root entity from the resource.
    let root_entity = root_node.0;

    let test_ui_container = commands
        .spawn((
            TestUiElement,
            Node,
            Style {
                position: taffy::style::Position::Absolute,
                width: Size::Percent(100.0),
                height: Size::Percent(100.0),
                justify_content: Some(taffy::JustifyContent::SpaceBetween),
                align_items: Some(taffy::AlignItems::Center),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            // Red box child
            parent
                .spawn((
                    Node,
                    Style {
                        width: Size::Percent(25.0),
                        height: Size::Percent(25.0),
                        justify_content: Some(taffy::JustifyContent::End),
                        align_items: Some(taffy::AlignItems::Center),
                        ..Default::default()
                    },
                    UiBackground::SolidColor { color: [1.0, 0.0, 0.0, 0.5] },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node,
                            Style {
                                width: Size::Percent(100.0),
                                height: Size::Percent(50.0),
                                justify_content: Some(taffy::JustifyContent::Center),
                                align_items: Some(taffy::AlignItems::Center),
                                ..Default::default()
                            },
                            UiBackground::SolidColor { color: [0.8, 0.9, 0.1, 0.5] },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Node,
                                Style {
                                    width: Size::Percent(100.0),
                                    height: Size::Percent(100.0),
                                    ..Default::default()
                                },
                                UiText {
                                    content: "Hello World Hello World Hello World Hello World Hello World Hello World Hello World Hello World".to_string(),
                                    font_size: 44.0,
                                    color: [0.5, 1.0, 0.5, 1.0],
                                    align: TextAlign::Center,
                                },
                            ));
                        });
                });

            // Green box child
            parent
                .spawn((
                    Node,
                    Style {
                        width: Size::Percent(25.0),
                        height: Size::Percent(25.0),
                        align_items: Some(taffy::AlignItems::Center),
                        justify_content: Some(taffy::JustifyContent::Center),
                        ..Default::default()
                    },
                    UiBackground::SolidColor { color: [0.0, 1.0, 0.0, 1.0] },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node,
                        Style {
                            width: Size::Percent(100.0),
                            height: Size::Percent(100.0),
                            ..Default::default()
                        },
                        UiText {
                            content: "Another Text Node 😈😈😈".to_string(),
                            font_size: 48.0,
                            color: [1.0, 1.0, 1.0, 1.0],
                            align: TextAlign::End,
                        },
                    ));
                });
        })
        .id();

    commands.entity(root_entity).add_child(test_ui_container);
}
