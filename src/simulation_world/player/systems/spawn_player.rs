use crate::{prelude::*, simulation_world::chunk::TransformComponent};
use bevy_ecs::prelude::*;
use glam::Quat;
use tracing::instrument;

#[derive(Component)]
pub struct PlayerMarkerComponent;

#[instrument(skip_all)]
pub fn spawn_player_system(
    // Input (ensure no player exists)
    player_query: Query<Entity, With<PlayerMarkerComponent>>,

    // Output
    mut commands: Commands,
) {
    if player_query.iter().next().is_some() {
        error!("Player already exists, not spawning another.");
        return;
    }

    info!("Spawning new player entity.");

    // Spawn them in a random location near the origin
    let random_position = Vec3::new(0.0, 5.0, 0.0);

    commands.spawn((
        PlayerMarkerComponent,
        TransformComponent {
            position: random_position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        },
    ));
}
