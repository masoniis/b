use super::{
    events::internal::WindowResizeEvent,
    resources::{Buttons, CursorMovement},
    systems::{processing, utils},
    ActionStateResource, InputActionMapResource,
};
use crate::{
    ecs_core::{state_machine::AppState, EcsBuilder, Plugin},
    simulation_world::{
        input::events::{
            KeyboardInputEvent, MouseButtonInputEvent, MouseMoveEvent, MouseScrollEvent,
            RawDeviceEvent, RawWindowEvent,
        },
        scheduling::OnExit,
        SimulationSchedule, SimulationSet,
    },
};
use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs};
use winit::{event::MouseButton, keyboard::PhysicalKey};

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
        builder.world.init_resource::<Events<RawWindowEvent>>();
        builder.world.init_resource::<Events<RawDeviceEvent>>();

        // Internal events (an ecs system fires them)
        builder.world.init_resource::<Events<KeyboardInputEvent>>();
        builder.world.init_resource::<Events<MouseMoveEvent>>();
        builder.world.init_resource::<Events<MouseScrollEvent>>();
        builder.world.init_resource::<Events<WindowResizeEvent>>();
        builder
            .world
            .init_resource::<Events<MouseButtonInputEvent>>();

        // Schedules
        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(
                (
                    processing::window_events_system,
                    processing::device_events_system,
                    processing::handle_resize_system.after(processing::window_events_system),
                    processing::update_action_state_system
                        .after(processing::window_events_system)
                        .after(processing::device_events_system),
                )
                    .in_set(SimulationSet::Input),
            );

        builder
            .schedule_entry(OnExit(AppState::StartingUp))
            .add_systems(utils::clear_stale_input_events_system);
    }
}
