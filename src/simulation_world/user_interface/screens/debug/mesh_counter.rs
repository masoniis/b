use crate::prelude::*;
use crate::simulation_world::chunk::MeshComponent;
use crate::simulation_world::user_interface::components::UiText;
use crate::simulation_world::user_interface::screens::debug::debug_screen::MeshCounterTextElementMarker;
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct MeshCounterResource(pub usize);

/// Updates the content of the Mesh counter text element.
#[instrument(skip_all)]
pub fn track_mesh_count_system(
    mut mesh_count: ResMut<MeshCounterResource>,

    // Input queries
    added_meshes: Query<(), Added<MeshComponent>>,
    removed_meshes: RemovedComponents<MeshComponent>,
) {
    // check for additions
    let added_count = added_meshes.iter().count();
    if added_count > 0 {
        mesh_count.0 += added_count;
    }

    // check for removals
    let removed_count = removed_meshes.len();
    if removed_count > 0 {
        mesh_count.0 -= removed_count;
    }
}

/// Updates the content of the Mesh counter text element when the resource changes.
#[instrument(skip_all)]
pub fn update_mesh_counter_system(
    // Input
    mesh_counter: Res<MeshCounterResource>,

    // Output (updated UI)
    mut ui_query: Query<&mut UiText, With<MeshCounterTextElementMarker>>,
) {
    if let Ok(mut text_component) = ui_query.single_mut() {
        text_component.content = format!("Mesh Counter: {}", mesh_counter.0);
    } else {
        warn!("MeshCounterTextElementMarker doesn't exist, but MeshCounter changed.");
    }
}
