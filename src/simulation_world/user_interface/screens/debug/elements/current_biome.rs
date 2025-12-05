use crate::simulation_world::{
    biome::BiomeRegistryResource,
    chunk::{ChunkCoord, ChunkStateManager},
    player::{ActiveCamera, CameraComponent},
    terrain::BiomeMapComponent,
    user_interface::{components::UiText, screens::debug::debug_screen::CurrentBiomeTextMarker},
};
use bevy_ecs::prelude::*;

pub fn update_current_biome_text_system(
    // Input
    active_camera: Res<ActiveCamera>,
    camera_query: Query<&CameraComponent>,
    chunk_state_manager: Res<ChunkStateManager>,
    biome_registry: Res<BiomeRegistryResource>,
    chunk_query: Query<&BiomeMapComponent>,
    // Output (updated ui text)
    mut query: Query<&mut UiText, With<CurrentBiomeTextMarker>>,
    // Local state to track the last displayed string to avoid allocations
    mut last_text_cache: Local<String>,
) {
    if let Ok(cam) = camera_query.get(active_camera.0) {
        if let Ok(mut ui_text) = query.single_mut() {
            // get the cam position and block
            let world_pos_ivec = cam.position.floor().as_ivec3();
            let (chunk_coord_pos, in_chunk_pos) =
                ChunkCoord::world_to_chunk_and_local_pos(world_pos_ivec);

            // update ui text with biome data based on chunk
            let new_text =
                if let Some(chunk_entity) = chunk_state_manager.get_entity(chunk_coord_pos) {
                    if let Ok(biome_map) = chunk_query.get(chunk_entity) {
                        let biome_id = biome_map.get_data_unchecked(
                            in_chunk_pos.x as usize,
                            in_chunk_pos.y as usize,
                            in_chunk_pos.z as usize,
                        );

                        let biome_def = biome_registry.get(biome_id);
                        biome_def.name.to_string()
                    } else {
                        "N/A (BiomeMap missing)".to_string()
                    }
                } else {
                    "N/A (Chunk not loaded)".to_string()
                };

            // only write to ui if we changed
            if *last_text_cache != new_text {
                *last_text_cache = new_text.clone();
                ui_text.content = new_text;
            }
        }
    }
}
