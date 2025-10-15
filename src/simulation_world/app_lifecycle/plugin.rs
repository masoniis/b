use crate::{
    ecs_core::{systems::apply_state_transition_system, EcsBuilder, Plugin, StatePlugin},
    simulation_world::{time::time_system, SimulationSchedule, SimulationSet},
};
use bevy_ecs::prelude::*;

use super::{
    finalize_loading_system, start_fake_work_system, update_app_time_system, AppState,
    AppTimeResource, GameState,
};

pub struct AppLifecyclePlugin;

impl Plugin for AppLifecyclePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(AppTimeResource::default());

        builder
            .add_plugin(StatePlugin::<AppState>::default())
            .add_plugin(StatePlugin::<GameState>::default());

        // The state resources the state machine oversees
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(start_fake_work_system);

        builder
            .schedule_entry(SimulationSchedule::Loading)
            .add_systems((finalize_loading_system, time_system).chain());

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (
                    apply_state_transition_system::<AppState>,
                    apply_state_transition_system::<GameState>,
                    update_app_time_system,
                )
                    .in_set(SimulationSet::PreUpdate),
            );
    }
}
