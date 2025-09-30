use super::super::super::components::{FpsCounterScreenTextMarker, ScreenTextComponent};
use crate::ecs_resources::TimeResource;
use bevy_ecs::prelude::*;

pub fn update_debug_diagnostics_system(
    time: Res<TimeResource>,
    mut query: Query<&mut ScreenTextComponent, With<FpsCounterScreenTextMarker>>,
) {
    if let Ok(mut text_component) = query.single_mut() {
        text_component.text = format!("FPS: {:.1}", time.smoothed_fps);
    }
}
