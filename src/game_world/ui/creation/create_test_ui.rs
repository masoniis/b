use crate::game_world::ui::components::{
    Children, Node, Parent, Size, Style, UiBackground, UiRoot, UiText,
};
use crate::prelude::*;
use bevy_ecs::prelude::*;

/// A system that tests the UI by spawning in a few UI entities
pub fn create_test_ui_system(mut commands: Commands) {
    info!("Spawning test UI");

    let text_entity = commands
        .spawn((
            Node,
            Style {
                width: Size::Auto,
                height: Size::Auto,
                ..Default::default()
            },
            UiText {
                content: "Hello World".to_string(),
                font_size: 24.0,
                color: [1.0, 1.0, 1.0, 1.0],
            },
        ))
        .id();

    let panel_entity = commands
        .spawn((
            Node,
            Style {
                width: Size::Percent(25.0),
                height: Size::Percent(25.0),
                justify_content: Some(taffy::JustifyContent::Center),
                align_items: Some(taffy::AlignItems::Center),
                ..Default::default()
            },
            UiBackground::SolidColor {
                color: [0.8, 0.1, 0.1, 0.5],
            },
            // It contains the text entity as its child.
            Children(vec![text_entity]),
        ))
        .id();

    commands.entity(text_entity).insert(Parent(panel_entity));

    let child2 = commands
        .spawn((
            Node,
            Style {
                width: Size::Percent(25.0),
                height: Size::Percent(25.0),
                ..Default::default()
            },
            UiBackground::SolidColor {
                color: [0.0, 1.0, 0.0, 0.2],
            },
        ))
        .id();

    commands.spawn((
        Node,
        UiRoot,
        Style {
            width: Size::Percent(100.0),
            height: Size::Percent(100.0),
            justify_content: Some(taffy::JustifyContent::SpaceBetween),
            align_items: Some(taffy::AlignItems::Center),
            ..Default::default()
        },
        Children(vec![panel_entity, child2]),
    ));
}
