use super::{systems::main, InputResource};
use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_resources::events::{KeyboardInputEvent, MouseInputEvent, MouseScrollEvent},
};
use bevy_ecs::{event::Events, schedule::IntoScheduleConfigs, world::World};

pub struct InputModuleBuilder;

impl Plugin for InputModuleBuilder {
    fn build(&self, schedules: &mut Schedules, world: &mut World) {
        world.insert_resource(InputResource::new());

        world.init_resource::<Events<KeyboardInputEvent>>();
        world.init_resource::<Events<MouseInputEvent>>();
        world.init_resource::<Events<MouseScrollEvent>>();

        schedules.main.add_systems((
            main::update_input_system.before(main::input_event_handler),
            main::input_event_handler,
        ));
    }
}
