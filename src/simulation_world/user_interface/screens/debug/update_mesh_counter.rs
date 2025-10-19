use crate::prelude::*;
use crate::simulation_world::chunk::MeshComponent;
use crate::simulation_world::user_interface::components::UiText;
use crate::simulation_world::user_interface::screens::debug::debug_screen::MeshCounterTextElementMarker;
use bevy_ecs::prelude::*;

/// Updates the content of the Mesh counter text element.
#[instrument(skip_all)]
pub fn update_mesh_counter_system(
    // System-local state
    mut mesh_count: Local<usize>,

    // Input queries
    mut query: Query<&mut UiText, With<MeshCounterTextElementMarker>>,
    added_meshes: Query<(), Added<MeshComponent>>,
    removed_meshes: RemovedComponents<MeshComponent>,
) {
    let mut has_changed = false;

    // check for additions
    let added_count = added_meshes.iter().count();
    if added_count > 0 {
        *mesh_count += added_count;
        has_changed = true;
    }

    // check for removals
    let removed_count = removed_meshes.len();
    if removed_count > 0 {
        *mesh_count -= removed_count;
        has_changed = true;
    }

    // update the UI if the count actually changed
    if has_changed {
        if let Ok(mut text_component) = query.single_mut() {
            text_component.content = format!("Mesh Counter: {}", *mesh_count);
        } else {
            error!("MeshCounterTextElementMarker should exist if diagnostic UI is visible!");
        }
    }
}
