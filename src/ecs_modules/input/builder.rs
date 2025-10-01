use super::{systems::main, ActionStateResource, InputActionMapResource, InputResource};
use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_modules::input::events::{
        KeyboardInputEvent, MouseButtonInputEvent, MouseMoveEvent, MouseScrollEvent,
        RawDeviceEvent, RawWindowEvent,
    },
};
use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs, world::World};

pub struct InputModuleBuilder;

impl Plugin for InputModuleBuilder {
    fn build(&self, schedules: &mut Schedules, world: &mut World) {
        world.insert_resource(InputResource::default());
        world.insert_resource(InputActionMapResource::default());
        world.insert_resource(ActionStateResource::default());

        // External events (comes from the app wrapper)
        world.init_resource::<Events<RawWindowEvent>>();
        world.init_resource::<Events<RawDeviceEvent>>();

        // Internal events (an ecs system fires them)
        world.init_resource::<Events<KeyboardInputEvent>>();
        world.init_resource::<Events<MouseMoveEvent>>();
        world.init_resource::<Events<MouseScrollEvent>>();
        world.init_resource::<Events<MouseButtonInputEvent>>();

        schedules.input.add_systems((
            main::input_event_system,
            main::update_action_state_system.after(main::input_event_system),
        ));
    }
}
