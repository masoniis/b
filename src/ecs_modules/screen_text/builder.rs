use crate::ecs_bridge::{Plugin, Schedules};
use bevy_ecs::{schedule::IntoScheduleConfigs, world::World};

use super::{main_system, startup_system};

pub struct ScreenTextModuleBuilder;

impl Plugin for ScreenTextModuleBuilder {
    fn build(&self, schedules: &mut Schedules, _world: &mut World) {
        schedules
            .startup
            .add_systems((startup_system::init_screen_diagnostics_system,));

        schedules.main.add_systems(
            main_system::handle_text_visibility_change_system
                .before(main_system::update_debug_diagnostics_system),
        );
        schedules.main.add_systems(
            main_system::update_visible_text_system
                .after(main_system::handle_text_visibility_change_system)
                .after(main_system::update_debug_diagnostics_system),
        );
        schedules.main.add_systems(
            main_system::removed_screen_text_system
                .after(main_system::update_debug_diagnostics_system),
        );
        schedules
            .main
            .add_systems(main_system::update_debug_diagnostics_system);
        schedules
            .main
            .add_systems(main_system::toggle_debug_diagnostics_system);
    }
}
