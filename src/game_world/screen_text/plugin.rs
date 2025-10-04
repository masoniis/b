use super::systems::{main, startup};
use crate::game_world::{schedules::GameSchedule, Plugin, ScheduleBuilder};
use bevy_ecs::world::World;

pub struct ScreenTextModulePlugin;

impl Plugin for ScreenTextModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, _world: &mut World) {
        schedules
            .entry(GameSchedule::Startup)
            .add_systems((startup::init_screen_diagnostics_system,));

        schedules.entry(GameSchedule::Main).add_systems((
            // main::handle_text_visibility_change_system
            //     .before(main::update_debug_diagnostics_system),
            // main::update_visible_text_system
            //     .after(main::handle_text_visibility_change_system)
            //     .after(main::update_debug_diagnostics_system),
            main::update_debug_diagnostics_system,
            main::toggle_debug_diagnostics_system,
        ));
    }
}
