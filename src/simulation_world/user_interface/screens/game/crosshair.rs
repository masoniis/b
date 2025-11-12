use crate::prelude::*;
use crate::simulation_world::user_interface::{
    components::{Node, Size, Style, UiBackground},
    screens::spawn_root::UiRootNodeResource,
};
use bevy_ecs::prelude::*;

pub fn spawn_crosshair(mut commands: Commands, root_node: Res<UiRootNodeResource>) {
    info!("Spawning crosshair element");

    let crosshair_entity = commands
        .spawn((
            Node,
            Style {
                width: Size::Auto,
                height: Size::Auto,
                position: taffy::style::Position::Absolute,
                justify_content: Some(taffy::style::JustifyContent::Center),
                align_items: Some(taffy::style::AlignItems::Center),
                ..Style::default()
            },
        ))
        .with_children(|parent| {
            // horizontal hair
            parent.spawn((
                Node,
                Style {
                    width: Size::Px(16.0),
                    height: Size::Px(4.0),
                    ..Style::default()
                },
                UiBackground::SolidColor {
                    color: [1.0, 1.0, 1.0, 0.8],
                },
            ));
            // vertical hair
            parent.spawn((
                Node,
                Style {
                    width: Size::Px(4.0),
                    height: Size::Px(16.0),
                    position: taffy::style::Position::Absolute,
                    ..Style::default()
                },
                UiBackground::SolidColor {
                    color: [1.0, 1.0, 1.0, 0.8],
                },
            ));
        })
        .id();

    commands.entity(root_node.0).add_child(crosshair_entity);
}
