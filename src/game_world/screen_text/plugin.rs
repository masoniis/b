use super::systems::{main, startup};
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    game_world::schedules::GameSchedule,
};

pub struct ScreenTextModulePlugin;

impl Plugin for ScreenTextModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(GameSchedule::Startup)
            .add_systems((startup::init_screen_diagnostics_system,));

        builder.schedule_entry(GameSchedule::Main).add_systems((
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
