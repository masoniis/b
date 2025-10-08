use super::{
    creation::create_test_ui_system,
    layout::{
        compute_and_apply_layout, compute_ui_depth_system, sync_ui_to_taffy_system, UiLayoutTree,
    },
    text::setup_font_system,
};
use crate::{
    ecs_core::{EcsBuilder, Plugin},
    game_world::schedules::GameSchedule,
};
use bevy_ecs::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.world.init_non_send_resource::<UiLayoutTree>();

        builder
            .schedule_entry(GameSchedule::Startup)
            .add_systems((create_test_ui_system, setup_font_system));

        builder.schedule_entry(GameSchedule::Main).add_systems(((
            sync_ui_to_taffy_system,
            compute_and_apply_layout,
            compute_ui_depth_system,
        )
            .chain(),));
    }
}
