use crate::prelude::*;
use crate::simulation_world::{
    global_resources::WorldTimeResource,
    user_interface::{
        components::UiText, screens::debug::debug_screen::FpsCounterTextElementMarker,
    },
};
use bevy_ecs::prelude::*;

/// Updates the content of the FPS counter text element.
pub fn update_fps_counter_system(
    // Input
    time: Res<WorldTimeResource>,

    // Output (updated component)
    mut query: Query<&mut UiText, With<FpsCounterTextElementMarker>>,
) {
    if let Ok(mut text_component) = query.single_mut() {
        text_component.content = format!("Fps: {:.2}", time.smoothed_fps);
    } else {
        error!("FpsCounterTextElementMarker should exist if diagnostic UI is visible!");
    }
}
