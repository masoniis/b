use crate::simulation_world::{
    terrain::ActiveTerrainGenerator,
    user_interface::{components::UiText, screens::debug::debug_screen::ActiveGenTextMarker},
};
use bevy_ecs::{
    change_detection::DetectChanges,
    prelude::{Query, Res, With},
};

pub fn update_active_gen_text_system(
    active_gen: Res<ActiveTerrainGenerator>,
    mut query: Query<&mut UiText, With<ActiveGenTextMarker>>,
) {
    if active_gen.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            text.content = active_gen.0.name().to_string();
        }
    }
}
