pub mod messages;
pub mod resources;
pub mod systems;
pub mod types;

pub use resources::{ActionStateResource, Buttons, CursorMovement, InputActionMapResource};
pub use types::*;

// INFO: -----------------------------
//         input module plugin
// -----------------------------------

use crate::{
    ecs_core::{state_machine::AppState, EcsBuilder, Plugin},
    simulation_world::{
        input::{
            messages::{
                internal::MouseResizeMessage, KeyboardInputMessage, MouseButtonInputMessage,
                MouseMoveMessage, MouseScrollMessage, RawDeviceMessage, RawWindowMessage,
            },
            resources::DesiredCursorState,
            systems::{
                raycast_break_voxel_event_system, toggle_chunk_borders::ChunkBoundsToggle,
                toggle_chunk_borders_system, toggle_cursor_system,
                toggle_opaque_wireframe::OpaqueWireframeMode, toggle_opaque_wireframe_mode_system,
            },
        },
        scheduling::OnExit,
        SimulationSchedule, SimulationSet,
    },
};
use bevy_ecs::{
    message::Messages,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::Res,
};
use systems::processing;
use winit::{event::MouseButton, keyboard::PhysicalKey};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    WindowEvents,
    DeviceEvents,
}

pub struct InputModulePlugin;

impl Plugin for InputModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // resources
        builder
            .add_resource(InputActionMapResource::default())
            .add_resource(ActionStateResource::default());

        builder
            .add_resource(Buttons::<PhysicalKey>::default())
            .add_resource(Buttons::<MouseButton>::default())
            .add_resource(CursorMovement::default());

        // external events (comes from the app wrapper)
        builder
            .init_resource::<Messages<RawWindowMessage>>()
            .init_resource::<Messages<RawDeviceMessage>>();

        // internal events (an ecs system fires them)
        builder
            .init_resource::<Messages<KeyboardInputMessage>>()
            .init_resource::<Messages<MouseMoveMessage>>()
            .init_resource::<Messages<MouseScrollMessage>>()
            .init_resource::<Messages<MouseResizeMessage>>()
            .init_resource::<Messages<MouseButtonInputMessage>>();

        // schedules
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                processing::window_events_system
                    .in_set(InputSystemSet::WindowEvents)
                    .in_set(SimulationSet::Input),
            )
            .add_systems(
                processing::device_events_system
                    .in_set(InputSystemSet::DeviceEvents)
                    .in_set(SimulationSet::Input),
            )
            .add_systems(
                processing::handle_resize_system
                    .after(InputSystemSet::WindowEvents)
                    .in_set(SimulationSet::Input),
            )
            .add_systems(
                processing::update_action_state_system
                    .after(InputSystemSet::WindowEvents)
                    .after(InputSystemSet::DeviceEvents)
                    .in_set(SimulationSet::Input),
            );

        builder
            .schedule_entry(OnExit(AppState::StartingUp))
            .add_systems(processing::clear_stale_input_events_system);

        // INFO: -------------------------------------
        //         keybind-based actions below
        // -------------------------------------------

        // set desired cursor state on pause action
        builder
            .add_resource(DesiredCursorState::default())
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                toggle_cursor_system.run_if(|action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::TogglePause)
                }),
            );

        // toggle opaque wireframe mode
        builder
            .add_resource(OpaqueWireframeMode::default())
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(toggle_opaque_wireframe_mode_system.run_if(
                |action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::ToggleOpaqueWireframeMode)
                },
            ));

        // toggle chunk borders
        builder
            .add_resource(ChunkBoundsToggle::default())
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(toggle_chunk_borders_system.run_if(
                |action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::ToggleChunkBorders)
                },
            ));

        // break voxel on click
        builder
            .add_resource(ChunkBoundsToggle::default())
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(raycast_break_voxel_event_system.run_if(
                |action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::BreakVoxel)
                },
            ));
    }
}
