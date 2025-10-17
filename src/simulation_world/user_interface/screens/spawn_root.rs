use crate::prelude::*;
use crate::simulation_world::user_interface::components::{Node, Size, Style};
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct UiRootNodeResource(pub Entity);

/// A system that spawns the single UI root node and registers it as a resource.
///
/// This should only run once at app startup.
#[instrument(skip_all)]
pub fn spawn_ui_root_system(mut commands: Commands) {
    info!("Setting up UI Root Node...");

    let root_entity = commands
        .spawn((
            Node,
            Style {
                position: taffy::style::Position::Absolute,
                width: Size::Percent(100.0),
                height: Size::Percent(100.0),
                ..Default::default()
            },
        ))
        .id();

    commands.insert_resource(UiRootNodeResource(root_entity));
}
