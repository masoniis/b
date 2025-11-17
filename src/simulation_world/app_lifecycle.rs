use crate::{
    ecs_core::{
        async_loading::{
            master_finalize_loading_system, poll_simulation_loading_tasks,
            reset_loading_tracker_system, start_fake_work_system, OnLoadComplete,
        },
        state_machine::{
            in_state, systems::apply_state_transition_system, AppState, GameState, StatePlugin,
        },
        EcsBuilder, Plugin,
    },
    simulation_world::{OnExit, SimulationSchedule, SimulationSet},
};
use bevy_ecs::schedule::IntoScheduleConfigs;

pub struct AppLifecyclePlugin;

/// A plugin for the simulation world that sets up the necessary
/// systems for handling the application lifecycle. This primarily
/// involves orchestration of loading tasks and state transitions.
impl Plugin for AppLifecyclePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------------
        //         async loading
        // -----------------------------

        // polling systems and tracking load state
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((poll_simulation_loading_tasks
                .in_set(SimulationSet::Update)
                .run_if(in_state(AppState::StartingUp)),));

        // load cleanup to run after transitions
        builder
            .schedule_entry(OnExit(AppState::StartingUp))
            .add_systems(reset_loading_tracker_system);

        // systems to ensure rigidity
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems(start_fake_work_system);

        // INFO: ---------------------------
        //         state transitions
        // ---------------------------------
        builder
            .add_plugin(StatePlugin::<AppState>::default())
            .add_plugin(StatePlugin::<GameState>::default());

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (
                    apply_state_transition_system::<AppState>,
                    master_finalize_loading_system::<AppState>,
                    apply_state_transition_system::<GameState>,
                    master_finalize_loading_system::<GameState>,
                )
                    .in_set(SimulationSet::PreUpdate),
            );

        // initial startup loading state should take us from loading
        // to running/playing once they finish.
        builder
            .add_resource(OnLoadComplete::new(AppState::Running))
            .add_resource(OnLoadComplete::new(GameState::Playing));
    }
}
