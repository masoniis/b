use bevy_ecs::schedule::{IntoScheduleConfigs, Schedule};
use bevy_ecs::world::World;

mod components;
pub use components::*;

mod main_system;
pub use main_system::*;

mod startup_system;
pub use startup_system::*;

pub struct ScreenTextModule;

impl ScreenTextModule {
    pub fn build(
        startup_schedule: &mut Schedule,
        main_schedule: &mut Schedule,
        _world: &mut World,
    ) {
        startup_schedule.add_systems((startup_system::init_screen_diagnostics_system,));

        main_schedule.add_systems((
            main_system::handle_text_visibility_change_system
                .before(main_system::update_debug_diagnostics_system),
            main_system::update_visible_text_system
                .after(main_system::handle_text_visibility_change_system)
                .after(main_system::update_debug_diagnostics_system),
            main_system::removed_screen_text_system
                .after(main_system::update_debug_diagnostics_system),
            main_system::update_debug_diagnostics_system,
            main_system::toggle_debug_diagnostics_system,
        ));
    }
}
