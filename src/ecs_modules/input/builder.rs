use super::{
    resources::{Buttons, CursorMovement},
    systems::main,
    ActionStateResource, InputActionMapResource,
};
use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_modules::input::events::{
        KeyboardInputEvent, MouseButtonInputEvent, MouseMoveEvent, MouseScrollEvent,
        RawDeviceEvent, RawWindowEvent,
    },
    ecs_modules::CoreSet,
};
use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs, world::World};
use winit::{event::MouseButton, keyboard::PhysicalKey};

pub struct InputModuleBuilder;

impl Plugin for InputModuleBuilder {
    fn build(&self, schedules: &mut Schedules, world: &mut World) {
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

        schedules.main.add_systems(
            (
                main::window_events_system,
                main::device_events_system,
                main::update_action_state_system
                    .after(main::window_events_system)
                    .after(main::device_events_system),
            )
                .in_set(CoreSet::Input),
        );
    }
}
