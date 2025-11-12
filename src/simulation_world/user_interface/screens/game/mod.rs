pub mod crosshair;

// INFO: ----------------
//         Plugin
// ----------------------

use crate::ecs_core::{EcsBuilder, Plugin};
use crate::prelude::*;
use crate::simulation_world::user_interface::screens::game::crosshair::spawn_crosshair;

pub struct GameScreenPlugin;

impl Plugin for GameScreenPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(OnEnter(GameState::Playing))
            .add_systems(spawn_crosshair);
    }
}
