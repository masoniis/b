use crate::game_world::ui::components::{Node, Size, Style, UiBackground, UiRoot, UiText};
use crate::prelude::*;
use bevy_ecs::prelude::*;

/// A system that tests the UI by spawning in a few UI entities using the builder pattern.
pub fn create_test_ui_system(mut commands: Commands) {
    info!("Spawning test UI with .with_children()");

    commands
        .spawn((
            Node,
            UiRoot,
            Style {
                width: Size::Percent(100.0),
                height: Size::Percent(100.0),
                justify_content: Some(taffy::JustifyContent::SpaceBetween),
                align_items: Some(taffy::AlignItems::Center),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
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
                    UiBackground::SolidColor {
                        color: [0.8, 0.1, 0.1, 0.5],
                    },
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
                            UiBackground::SolidColor {
                                color: [0.8, 0.9, 0.1, 0.5],
                            },
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
                                    color: [1.0, 1.0, 1.0, 1.0],
                                },
                            ));
                        });
                });
            parent.spawn((
                Node,
                Style {
                    width: Size::Percent(25.0),
                    height: Size::Percent(25.0),
                    ..Default::default()
                },
                UiBackground::SolidColor {
                    color: [0.0, 1.0, 1.0, 0.5],
                },
            ));
        });
}
