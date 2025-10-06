use crate::game_world::ui::components::{Children, Node, Size, Style, UiMaterial, UiRoot};
use crate::prelude::*;
use bevy_ecs::prelude::*;

/// A system that tests the UI by spawning in a few UI entities
pub fn test_ui_system(mut commands: Commands) {
    info!("Spawning test UI");

    commands.spawn((
        Node,
        UiRoot,
        Style {
            width: Size::Percent(50.0),
            height: Size::Percent(50.0),
        },
        UiMaterial::SolidColor {
            color: [1.0, 1.0, 0.0, 0.2],
        },
        Children::default(),
    ));

    commands.spawn((
        Node,
        UiRoot,
        Style {
            width: Size::Percent(40.0),
            height: Size::Percent(30.0),
        },
        UiMaterial::SolidColor {
            color: [1.0, 0.0, 0.0, 0.2],
        },
        Children::default(),
    ));
}
