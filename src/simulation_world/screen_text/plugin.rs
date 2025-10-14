use super::systems::{main, startup};
use crate::ecs_core::{EcsBuilder, Plugin};
use crate::simulation_world::SimulationSchedule;

pub struct ScreenTextModulePlugin;

impl Plugin for ScreenTextModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .schedule_entry(SimulationSchedule::Startup)
            .add_systems((startup::init_screen_diagnostics_system,));

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((
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
