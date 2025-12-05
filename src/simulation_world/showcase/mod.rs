pub mod systems;

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        input::{types::simulation_action::SimulationAction, ActionStateResource},
        scheduling::SimulationSchedule,
    },
};
use bevy_ecs::prelude::{IntoScheduleConfigs, Res};
use systems::apply_showcase_system;

pub struct ShowcasePlugin;

impl Plugin for ShowcasePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                apply_showcase_system.run_if(|action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::Showcase1)
                        || action_state.just_happened(SimulationAction::Showcase2)
                        || action_state.just_happened(SimulationAction::Showcase3)
                }),
            );
    }
}
