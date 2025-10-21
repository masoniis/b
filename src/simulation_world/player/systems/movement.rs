use crate::prelude::*;
use crate::simulation_world::chunk::TransformComponent;
use crate::simulation_world::input::ActionStateResource;
use crate::simulation_world::input::SimulationAction;
use crate::simulation_world::player::systems::PlayerMarkerComponent;
use bevy_ecs::prelude::*;
use tracing::instrument;

#[instrument(skip_all)]
pub fn player_movement_system(
    // Input
    action_state: Res<ActionStateResource>,

    // Output (updated position)
    mut player_query: Query<(Entity, &mut TransformComponent), With<PlayerMarkerComponent>>,
) {
    let Ok((_, mut player_transform)) = player_query.single_mut() else {
        error!("No player entity or multiple player entities found for movement system.");
        return;
    };

    // INFO: -------------------------------
    //         Update position logic
    // -------------------------------------

    if action_state.is_ongoing(SimulationAction::MoveForward) {
        player_transform.position.z -= 0.1;
    }
}
