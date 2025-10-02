use crate::ecs_bridge::{Plugin, Schedules};
use bevy_ecs::{schedule::IntoScheduleConfigs, world::World};

use super::systems::{main, startup};

pub struct ScreenTextModuleBuilder;

impl Plugin for ScreenTextModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        schedules
            .startup
            .add_systems((startup::init_screen_diagnostics_system,));

        schedules.main.add_systems((
            main::handle_text_visibility_change_system
                .before(main::update_debug_diagnostics_system),
            main::update_visible_text_system
                .after(main::handle_text_visibility_change_system)
                .after(main::update_debug_diagnostics_system),
            main::update_debug_diagnostics_system,
            main::toggle_debug_diagnostics_system,
        ));
    }
}
