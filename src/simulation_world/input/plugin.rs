use super::{
    messages::internal::MouseResizeMessage,
    resources::{Buttons, CursorMovement},
    systems::{processing, utils},
    ActionStateResource, InputActionMapResource,
};
use crate::{
    ecs_core::{state_machine::AppState, EcsBuilder, Plugin},
    simulation_world::{
        input::messages::{
            KeyboardInputMessage, MouseButtonInputMessage, MouseMoveMessage, MouseScrollMessage,
            RawDeviceMessage, RawWindowMessage,
        },
        scheduling::OnExit,
        SimulationSchedule, SimulationSet,
    },
};
use bevy_ecs::{
    message::Messages,
    schedule::{IntoScheduleConfigs, SystemSet},
};
use winit::{event::MouseButton, keyboard::PhysicalKey};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    WindowEvents,
    DeviceEvents,
}

pub struct InputModulePlugin;

impl Plugin for InputModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // Resources
        builder.add_resource(InputActionMapResource::default());
        builder.add_resource(ActionStateResource::default());

        builder.add_resource(Buttons::<PhysicalKey>::default());
        builder.add_resource(Buttons::<MouseButton>::default());
        builder.add_resource(CursorMovement::default());

        // External events (comes from the app wrapper)
        builder.world.init_resource::<Messages<RawWindowMessage>>();
        builder.world.init_resource::<Messages<RawDeviceMessage>>();

        // Internal events (an ecs system fires them)
        builder
            .world
            .init_resource::<Messages<KeyboardInputMessage>>();
        builder.world.init_resource::<Messages<MouseMoveMessage>>();
        builder
            .world
            .init_resource::<Messages<MouseScrollMessage>>();
        builder
            .world
            .init_resource::<Messages<MouseResizeMessage>>();
        builder
            .world
            .init_resource::<Messages<MouseButtonInputMessage>>();

        // Schedules
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
            .add_systems(utils::clear_stale_input_events_system);
    }
}
