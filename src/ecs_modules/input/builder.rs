use super::{systems::main, ActionStateResource, InputActionMapResource, InputResource};
use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_resources::events::{KeyboardInputEvent, MouseInputEvent, MouseScrollEvent},
};
use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs, world::World};

pub struct InputModuleBuilder;

impl Plugin for InputModuleBuilder {
    fn build(&self, schedules: &mut Schedules, world: &mut World) {
        world.insert_resource(InputResource::new());
        world.insert_resource(InputActionMapResource::default());
        world.insert_resource(ActionStateResource::default());

        world.init_resource::<Events<KeyboardInputEvent>>();
        world.init_resource::<Events<MouseInputEvent>>();
        world.init_resource::<Events<MouseScrollEvent>>();

        schedules.main.add_systems((
            main::reset_input_state_system.before(main::input_event_handler),
            main::input_event_handler,
            main::update_action_state_system.after(main::input_event_handler),
        ));
    }
}
