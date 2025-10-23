use crate::prelude::*;
use crate::simulation_world::asset_management::{
    AssetStorageResource, MeshAsset, MeshComponentRemovedMessage,
};
use crate::simulation_world::chunk::MeshComponent;
use crate::simulation_world::user_interface::components::UiText;
use crate::simulation_world::user_interface::screens::debug_screen::{
    IndexCountTextMarker, MeshCountTextMarker, VertexCountTextMarker,
};
use bevy_ecs::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct MeshCounterResource {
    pub total_meshes: usize,
    pub total_vertices: usize,
    pub total_indices: usize,
}

/// Updates the content of the `MeshCounterResource`
#[instrument(skip_all)]
pub fn update_mesh_stats_system(
    // Inputs
    added_meshes: Query<&MeshComponent, Added<MeshComponent>>,
    mut removed_events: MessageReader<MeshComponentRemovedMessage>,

    // Outputs
    mut mesh_count: ResMut<MeshCounterResource>,
    asset_storage: Res<AssetStorageResource<MeshAsset>>,
) {
    // handle additions
    for mesh_component in added_meshes.iter() {
        if let Some(mesh) = asset_storage.get(mesh_component.mesh_handle) {
            mesh_count.total_meshes += 1;
            mesh_count.total_vertices += mesh.vertices.len();
            mesh_count.total_indices += mesh.indices.len();
        } else {
            warn!(
                "MeshComponent added with an invalid handle: {:?}",
                mesh_component.mesh_handle.id()
            );
        }
    }

    // handle removals
    for event in removed_events.read() {
        if let Some(mesh) = asset_storage.get(event.mesh_handle) {
            // use saturating_sub to prevent panicking if counts somehow mismatch
            mesh_count.total_meshes = mesh_count.total_meshes.saturating_sub(1);
            mesh_count.total_vertices = mesh_count
                .total_vertices
                .saturating_sub(mesh.vertices.len());
            mesh_count.total_indices = mesh_count.total_indices.saturating_sub(mesh.indices.len());
        } else {
            warn!(
                "MeshComponentRemovedMessage received for an invalid handle: {:?}",
                event.mesh_handle.id()
            );
        }
    }
}

/// Updates the content of the Mesh counter text element when the resource changes.
#[instrument(skip_all)]
pub fn update_mesh_counter_screen_text_system(
    // Input
    mesh_counter: Res<MeshCounterResource>,

    // Output (updated UI)
    mut text_query: Query<(
        &mut UiText,
        Option<&MeshCountTextMarker>,
        Option<&VertexCountTextMarker>,
        Option<&IndexCountTextMarker>,
    )>,
) {
    for (mut text, mesh_marker, vertex_marker, index_marker) in text_query.iter_mut() {
        if mesh_marker.is_some() {
            text.content = format!("{}", mesh_counter.total_meshes);
        } else if vertex_marker.is_some() {
            text.content = format!("{}", mesh_counter.total_vertices);
        } else if index_marker.is_some() {
            text.content = format!("{}", mesh_counter.total_indices);
        }
    }
}
