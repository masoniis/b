use super::{systems::main, ActionStateResource, InputActionMapResource, InputResource};
use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_modules::input::events::{
        keyboard_input_event::KeyboardInputEvent, mouse_button_input_event::MouseButtonInputEvent,
        mouse_input_event::MouseMoveEvent, mouse_scroll_event::MouseScrollEvent,
    },
};
use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs, world::World};

pub struct InputModuleBuilder;

impl Plugin for InputModuleBuilder {
    fn build(&self, schedules: &mut Schedules, world: &mut World) {
        world.insert_resource(InputResource::default());
        world.insert_resource(InputActionMapResource::default());
        world.insert_resource(ActionStateResource::default());

        world.init_resource::<Events<KeyboardInputEvent>>();
        world.init_resource::<Events<MouseMoveEvent>>();
        world.init_resource::<Events<MouseScrollEvent>>();
        world.init_resource::<Events<MouseButtonInputEvent>>();

        schedules.input.add_systems((
            main::reset_input_state_system.before(main::input_event_handler),
            main::input_event_handler,
            main::update_action_state_system.after(main::input_event_handler),
        ));
    }
}
