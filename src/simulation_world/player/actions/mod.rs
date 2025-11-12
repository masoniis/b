pub mod raycast_break_voxel;

// INFO: -------------------------------
//         actions module plugin
// -------------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        input::ActionStateResource,
        player::raycast_break_voxel::{break_targeted_voxel_system, update_targeted_block_system},
        SimulationSchedule,
    },
    SimulationAction,
};
use bevy_ecs::{
    schedule::{IntoScheduleConfigs, SystemSet},
    system::Res,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    WindowEvents,
    DeviceEvents,
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // break voxel on click
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(break_targeted_voxel_system.run_if(
                |action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::BreakVoxel)
                },
            ));

        // update targeted block
        builder
            .schedule_entry(SimulationSchedule::FixedUpdate)
            .add_systems(update_targeted_block_system);
    }
}
