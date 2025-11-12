pub mod voxel;

pub use voxel::*;

// INFO: -------------------------------
//         actions module plugin
// -------------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        input::ActionStateResource,
        player::{
            break_targeted_voxel::{handle_break_voxel_events_system, BreakVoxelEvent},
            place_voxel_at_target::{
                handle_place_voxel_events_system, place_targeted_voxel_system, PlaceVoxelEvent,
            },
        },
        SimulationSchedule,
    },
    SimulationAction, SimulationSet,
};
use bevy_ecs::{
    message::Messages,
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
        // update targeted block
        builder
            .schedule_entry(SimulationSchedule::FixedUpdate)
            .add_systems(update_targeted_block_system);

        // break voxel on click
        builder
            .init_resource::<Messages<BreakVoxelEvent>>()
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((
                handle_break_voxel_events_system,
                break_targeted_voxel_system
                    .in_set(SimulationSet::Update)
                    .run_if(|action_state: Res<ActionStateResource>| {
                        action_state.just_happened(SimulationAction::BreakVoxel)
                    }),
            ));

        // add voxel on right click
        builder
            .init_resource::<Messages<PlaceVoxelEvent>>()
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((
                handle_place_voxel_events_system,
                place_targeted_voxel_system
                    .in_set(SimulationSet::Update)
                    .run_if(|action_state: Res<ActionStateResource>| {
                        action_state.just_happened(SimulationAction::PlaceVoxel)
                    }),
            ));
    }
}
