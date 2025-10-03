use super::{
    resources::{Buttons, CursorMovement},
    systems::{processing, utils},
    ActionStateResource, InputActionMapResource,
};
use crate::game_world::{
    input::events::{
        KeyboardInputEvent, MouseButtonInputEvent, MouseMoveEvent, MouseScrollEvent,
        RawDeviceEvent, RawWindowEvent,
    },
    schedules::OnExit,
    state_machine::resources::AppState,
    CoreSet, Plugin, Schedules,
};
use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs, world::World};
use winit::{event::MouseButton, keyboard::PhysicalKey};

pub struct InputModulePlugin;

impl Plugin for InputModulePlugin {
    fn build(&self, schedules: &mut Schedules, world: &mut World) {
        // Resources
        world.insert_resource(InputActionMapResource::default());
        world.insert_resource(ActionStateResource::default());

        world.insert_resource(Buttons::<PhysicalKey>::default());
        world.insert_resource(Buttons::<MouseButton>::default());
        world.insert_resource(CursorMovement::default());

        // External events (comes from the app wrapper)
        world.init_resource::<Events<RawWindowEvent>>();
        world.init_resource::<Events<RawDeviceEvent>>();

        // Internal events (an ecs system fires them)
        world.init_resource::<Events<KeyboardInputEvent>>();
        world.init_resource::<Events<MouseMoveEvent>>();
        world.init_resource::<Events<MouseScrollEvent>>();
        world.init_resource::<Events<MouseButtonInputEvent>>();

        // Schedules
        schedules.main.add_systems(
            (
                processing::window_events_system,
                processing::device_events_system,
                processing::update_action_state_system
                    .after(processing::window_events_system)
                    .after(processing::device_events_system),
            )
                .in_set(CoreSet::Input),
        );

        schedules
            .get_labeled_mut(OnExit(AppState::Loading))
            .add_systems(utils::clear_stale_input_events_system);
    }
}
